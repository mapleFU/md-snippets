#![feature(wrapping_int_impl)]
#![feature(assoc_char_funcs)]

/// url shortener 利用 snowflake 生成 hash, 然后取 short url
mod storage;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::Wrapping;

use snowflake::Node as SnowflakeNode;
use bloom_filter::BloomFilter;
// use bloom_filter::Config as BloomFilterConfig;
use crate::storage::Error as StorageError;

use thiserror::Error;

pub struct UrlShortenerConfig {
    pub url_length: usize,
    pub snowflake_id: i32,

    pub use_bloom_filter: bool,
}

pub struct UrlShortener {
    conf: UrlShortenerConfig,

    uuid_helper: SnowflakeNode,
    storage: Box<dyn storage::Storage>,

    /// 注: 只有单机处理且是内存处理的时候，才能设置 bf，算是一个很奇怪的优化了
    #[allow(unused)]
    bloom_filter: Option<BloomFilter>,
}

impl UrlShortener {
    pub fn new(conf: UrlShortenerConfig) -> Result<Self> {
        let snowflake = SnowflakeNode::new(conf.snowflake_id)?;
        let storage = Box::new(storage::HashMapStorage::default());

        let bloom_filter = if conf.use_bloom_filter {
            unimplemented!()
        } else {
            None
        };

        Ok(Self {
            conf,
            uuid_helper: snowflake,
            storage,

            bloom_filter,
        })
    }
}

// see comment for readable_fn.
const READBLE_LENGTH: usize = 62;

/// generate readable string from `origin_hash` with `length`.
///
/// [48, 57] means digits.
/// [65, 90] means alphabet lower case.
/// [97, 192] means alphabet upper case.
fn readable_fn(length: usize, origin_hash: u64) -> String {
    let mut s = String::with_capacity(length);
    let mut current_remain = origin_hash;
    for _ in 0..length {
        let current = current_remain % READBLE_LENGTH as u64;

        let c: u32 = match current {
            v_digit @ 0..=9 => 48 + v_digit as u32,
            v_lower_alpha @ 10..=35 => v_lower_alpha as u32 - 10 + 65,
            v_upper_alpha @ 36..=61 => v_upper_alpha as u32 - 36 + 97,
            _ => {
                unreachable!()
            }
        };
        s.push(char::from_u32(c).unwrap());

        current_remain = current_remain / READBLE_LENGTH as u64;
    }
    s
}

impl UrlShortener {
    /// If the request has some error, return Err(error). Else
    /// If long url exists, return OK(long_url).
    /// If long url unexists, return OK(None).
    pub fn get_long_url(&self, short_url: impl AsRef<str>) -> Result<Option<String>> {
        Ok(self.storage.get_content(short_url.as_ref())?)
    }

    /// Generate short url for long_url.
    /// This will generate readable_fn(hash(long_url + uuid * duplicate_cnt)).
    ///
    /// TODO(mwish): Do we need to handle extra timeout?
    pub fn generate_short_url(&mut self, long_url: impl AsRef<str>) -> Result<String> {
        let mut t = DefaultHasher::new();
        long_url.as_ref().hash(&mut t);
        let mut base_id = Wrapping(t.finish());
        // TODO(mwish): Do we need to handle extra timeout?
        loop {
            let s = readable_fn(self.conf.url_length, base_id.0);
            let opt = self.storage.set_if_absent(long_url.as_ref(), s.as_ref())?;
            if opt.is_none() {
                base_id += Wrapping(self.uuid_helper.generate().0 as u64);
                continue;
            }
            return Ok(s);
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("storage error")]
    StorageError(#[from] StorageError),

    #[error("snowflake error")]
    SnowflakeError(#[from] std::time::SystemTimeError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readable_fn() {
        let strs = vec!["nmsl", "wsngg", "wmldsnmdez"];
        let lengths = vec![4, 6, 8, 10];
        for l in lengths {
            for s in &strs {
                let mut t = DefaultHasher::new();
                s.hash(&mut t);
                let hash_v = t.finish();
                let s = readable_fn(l, hash_v);
                assert_eq!(s.len(), l);
                println!("{:?}", s);
            }
        }
    }

    #[test]
    fn test_basic_hash_shortener() {
        let conf = UrlShortenerConfig {
            url_length: 8,
            snowflake_id: 2,
        };
        let mut shortener = UrlShortener::new(conf).unwrap();

        assert_eq!(shortener.get_long_url("nmsl").unwrap(), None);
        let s = shortener.generate_short_url("nmsl").unwrap();
        assert!(s.len() == 8);

        assert_eq!(
            shortener.get_long_url(&s).unwrap(),
            Some("nmsl".to_string())
        );

        // construct duplication
        let s2 = shortener.generate_short_url("nmsl").unwrap();
        assert!(s2.len() == 8);
        assert_eq!(
            shortener.get_long_url(&s2).unwrap(),
            Some("nmsl".to_string())
        );

        assert_ne!(s, s2);
    }
}
