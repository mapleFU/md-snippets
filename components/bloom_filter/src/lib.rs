#![feature(wrapping_int_impl)]

use bitvec::boxed::BitBox;
use bitvec::prelude::*;

use byteorder::{BigEndian, ReadBytesExt};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::Wrapping;

use std::io::{Cursor, Read};

/// A Bloom filter with a 1% error and an optimal value of k,
/// in contrast, requires only about 9.6 bits per element,
/// regardless of the size of the elements.
/// This advantage comes partly from its compactness,
/// inherited from arrays, and partly from its probabilistic nature.
/// The 1% false-positive rate can be reduced by a factor of
/// ten by adding only about 4.8 bits per element.
pub struct Config {
    /// 每个 key 平均的 bits 数目, 最小为 1, 最大为 30
    bits_per_key: usize,
    /// 估计中的 key 的数目
    estimate_key_number: usize,
}

impl Config {
    /// 根据 false positive rate 来生成 Config
    ///
    /// http://pages.cs.wisc.edu/~cao/papers/summary-cache/node8.html
    /// https://github.com/willf/bloom/blob/master/bloom.go
    pub fn new_with_estimate(false_positive: f64, estimate_key_number: usize) -> Self {
        Config {
            bits_per_key: 10,
            estimate_key_number,
        }
    }
}

/// If m is the number of bits in the array
/// The number of hash functions is k,
/// n is the number of string. To minimize false positive rate, k is (m / n) * ln2.
pub struct BloomFilter {
    cfg: Config,

    current_key: usize,
    hash_func_number: u8,
    bit_size: usize,
    bit_vec: BitBox,
}

impl BloomFilter {
    pub fn new(cfg: Config) -> Self {
        let mut hash_func_number =
            ((cfg.estimate_key_number as f64) * (cfg.bits_per_key as f64)) as u64;
        if hash_func_number > 30 {
            hash_func_number = 30;
        } else if hash_func_number < 1 {
            hash_func_number = 1;
        }

        let bits_size: usize = {
            if cfg.bits_per_key * cfg.estimate_key_number > 64 {
                cfg.bits_per_key * cfg.estimate_key_number
            } else {
                64
            }
        };
        BloomFilter {
            cfg,
            current_key: 0,
            // Note: hash_func_number is in [1, 30], so it's safe to as u8.
            hash_func_number: hash_func_number as u8,
            bit_size: bits_size,
            /// 我看了半天文档没看懂 bit_vec 咋回事
            bit_vec: bitbox![0; bits_size],
        }
    }

    /// dump_filter will dump a BloomFilter to String
    pub fn dump_filter(&self) -> Vec<u8> {
        // let mut slice: Vec<usize> = self.bit_vec.as_slice().to_owned();
        // slice.push(self.current_key);
        let slice: Vec<u16> = vec![];
        let resp_sz = self.current_key * (std::mem::size_of::<usize>() / std::mem::size_of::<u8>());
        let mut resp: Vec<u8> = Vec::with_capacity(resp_sz);

        let mut rdr = Cursor::new(slice);
        for _ in 0..resp_sz {
            resp.push(rdr.read_u8().unwrap());
        };

        resp
    }

    pub fn parse<S: Into<String>>(s: S) -> Self {
        unimplemented!()
    }

    pub fn add<T: AsRef<str>>(&mut self, key: T) {
        self.current_key += 1;
        let mut base_hash = Wrapping(bloom_hash(key));
        // let delta = (base_hash >> 33) | (base_hash << 31);
        let delta = base_hash.rotate_right(33);
        for _ in 0..self.hash_func_number {
            let bit_pos: usize = base_hash.0 as usize % self.bit_size;
            unsafe {
                *self.bit_vec.get_unchecked_mut(bit_pos) = true;
            }
            base_hash += delta;
        }
    }

    pub fn key_may_match<T: AsRef<str>>(&self, key: T) -> bool {
        let mut base_hash = Wrapping(bloom_hash(key));
        // let delta = (base_hash >> 33) | (base_hash << 31); // Rotate right 33 bits
        let delta = base_hash.rotate_right(33);
        for _ in 0..self.hash_func_number {
            let bit_pos: usize = base_hash.0 as usize % self.bit_size;
            if unsafe { !*self.bit_vec.get_unchecked(bit_pos) } {
                return false;
            }
        }
        true
    }
}

fn bloom_hash(s: impl AsRef<str>) -> u64 {
    let mut t = DefaultHasher::new();
    s.as_ref().hash(&mut t);
    t.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let cfg = Config::new_with_estimate(0.2, 100);
        let mut bloom = BloomFilter::new(cfg);

        assert!(!bloom.key_may_match("nmsl"));

        bloom.add("nmsl");

        assert!(bloom.key_may_match("nmsl"));
    }
}
