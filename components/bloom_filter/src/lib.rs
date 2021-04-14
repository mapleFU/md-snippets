#![feature(wrapping_int_impl)]

use bitvec::boxed::BitBox;
use bitvec::prelude::*;

use byteorder::{BigEndian, WriteBytesExt};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::Wrapping;

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
    /// bloom filter 的 bit_size
    bloom_size: usize,
}

impl Config {
    /// 根据 false positive rate 来生成 Config
    ///
    /// http://pages.cs.wisc.edu/~cao/papers/summary-cache/node8.html
    /// https://github.com/willf/bloom/blob/master/bloom.go
    pub fn new_with_estimate(false_positive: f64, estimate_key_number: usize) -> Self {
        let bloom_size = (-1.0_f64 * (estimate_key_number as f64) * false_positive.log2()
            / (2.0_f64).log2().powi(2))
        .ceil() as usize;
        
        Config {
            bits_per_key: 0,
            estimate_key_number,
            bloom_size,
        }
    }
}

/// If m is the number of bits in the array
/// The number of hash functions is k,
/// n is the number of string. To minimize false positive rate, k is (m / n) * ln2.
pub struct BloomFilter {
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
            if cfg.bloom_size >= cfg.estimate_key_number {
                cfg.bloom_size
            } else if cfg.bits_per_key * cfg.estimate_key_number > 64 {
                cfg.bits_per_key * cfg.estimate_key_number
            } else {
                64
            }
        };
        BloomFilter {
            current_key: 0,
            // Note: hash_func_number is in [1, 30], so it's safe to as u8.
            hash_func_number: hash_func_number as u8,
            bit_size: bits_size,
            /// 我看了半天文档没看懂 bit_vec 咋回事
            bit_vec: bitbox![0; bits_size],
        }
    }

    /// dump_filter will dump a BloomFilter to String
    ///
    /// Note: 这个地方实现的时候踩了一把 byteorder 和 usize 的坑。
    /// * Vec<u8>: u8 串 + 函数数目(1byte) + 现有 key 数目(4bytes)
    pub fn dump_filter(&self) -> Vec<u8> {
        let reserve_bytes = ((self.bit_size + 7) / 8) * 8;
        // 长度: u8 串 + 函数数目 + 现有 key 数目
        let resp_sz = reserve_bytes + (std::mem::size_of::<usize>() / std::mem::size_of::<u8>()) + 1;
        let mut resp: Vec<u8> = Vec::with_capacity(resp_sz);

        // TODO(mwish): https://stackoverflow.com/questions/53161339/how-do-i-use-the-byteorder-crate-to-read-a-usize
        // Note: 这里目前确保是 BigEndian
        let shorts = unsafe {
            let (prefix, shorts, surfix) = self.bit_vec.as_slice().align_to::<u8>();
            assert!(prefix.len() == 0);
            assert!(surfix.len() == 0);
            shorts
        };
        for i in 0..reserve_bytes {
            resp.push(shorts[i]);
        }

        resp.push(self.hash_func_number);

        resp.write_u64::<BigEndian>(self.current_key as u64).unwrap();

        resp
    }

    pub fn parse(s: impl AsRef<[u8]>) -> Self {
        let slice = s.as_ref();
        if slice.len() <= (std::mem::size_of::<usize>() / std::mem::size_of::<u8>()) + 1 {
            panic!("when parsing, the slice is too short");
        }
        unimplemented!();
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
            base_hash += delta;
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
        let cfg = Config::new_with_estimate(0.01, 100);
        let mut bloom = BloomFilter::new(cfg);

        assert!(!bloom.key_may_match("nmsl"));

        bloom.add("nmsl");

        assert!(bloom.key_may_match("nmsl"));

        assert!(!bloom.key_may_match("n"));
        assert!(!bloom.key_may_match("m"));
        assert!(!bloom.key_may_match("s"));
        assert!(!bloom.key_may_match("l"));
    }
}
