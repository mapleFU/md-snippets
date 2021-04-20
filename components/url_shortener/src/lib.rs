#![feature(wrapping_int_impl)]

/// url shortener 利用 snowflake 生成 hash, 然后取 short url

mod storage;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::Wrapping;

use thiserror::Error;
use snowflake::Node as SnowflakeNode;


use crate::storage::Error as StorageError;

pub struct UrlShortenerConfig {
    pub url_length: usize,
}

pub struct UrlShortener {
    conf: UrlShortenerConfig,

    uuid_helper: SnowflakeNode,
    storage: Box<dyn storage::Storage>,
}

fn readable_fn(length: usize, origin_hash: u64) -> String {
    unimplemented!()
}

impl UrlShortener {
    /// If the request has some error, return Err(error). Else
    /// If long url exists, return OK(long_url).
    /// If long url unexists, return OK(None).
    pub fn get_long_url(&self, short_url: impl AsRef<str>) -> Result<Option<String>> {
        unimplemented!()
    }

    /// Generate short url for long_url.
    /// This will generate readable_fn(hash(long_url + uuid * duplicate_cnt)).
    ///
    /// TODO(mwish): Do we need to handle extra timeout.
    pub fn generate_short_url(&mut self, long_url: impl AsRef<str>) -> Result<String> {
        let mut to_handle = long_url.as_ref().to_owned();

        let mut t = DefaultHasher::new();
        to_handle.as_str().hash(&mut t);
        let mut base_id = Wrapping(t.finish());
        /// TODO(mwish): Do we need to handle extra timeout?
        loop {
            let s = readable_fn(self.conf.url_length, base_id.0);
            let opt = self.storage.set_if_absent(s.as_ref(), long_url.as_ref())?;
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
    SnowflakeError,
}

pub type Result<T> = std::result::Result<T, Error>;
