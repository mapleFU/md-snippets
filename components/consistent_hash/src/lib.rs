use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use std::collections::HashMap;
use std::mem;

use byteorder::{LittleEndian, WriteBytesExt};

struct ConsistentHash {
    hash_fn: Box<dyn Fn(&[u8]) -> u64>,
    // Note: replicas should be no less than 1.
    replicas: u32,

    // The hash keys include the logic for replicas.
    // Note: should be ordered.
    hash_keys: Vec<u64>,

    hash_map: HashMap<u64, String>
}

impl ConsistentHash {
    pub fn is_empty(&self) -> bool {
        self.hash_map.is_empty()
    }

    pub fn add_keys(&mut self, keys: &[&str]) {
        for k in keys.iter() {
            for i in 0..self.replicas {
                let mut bytes: Vec<u8> = Vec::with_capacity(k.len() as usize + mem::size_of::<u64>() as usize);
                bytes.write_u64::<LittleEndian>(i as u64).expect("Unable to write");
                bytes.extend_from_slice(k.as_bytes());
                let hash = (self.hash_fn)(&bytes);
                // TODO(mwish): why is *k?
                self.hash_map.insert(hash, (*k).to_string());

                self.hash_keys.push(hash);
            }
        }

        self.hash_keys.sort();
    }

    pub fn get(&self, key: &str) -> String {
        let hash_val = (self.hash_fn)(key.as_bytes());

        let index;
        match self.hash_keys.binary_search(&hash_val) {
            Ok(v) => {
                index = v;
            },
            Err(v) => {
                index = v;
            }
        }

        return self.hash_map.get(&self.hash_keys[index]).unwrap().to_owned();
    }
}

struct ConsistentHashBuilder {
    replicas: u32,
    hash_fn: Box<dyn Fn(&[u8]) -> u64>,
}

const DEFAULT_HASH_REPLICAS: u32 = 7;

impl Default for ConsistentHashBuilder {
    fn default() -> Self { 
        ConsistentHashBuilder {
            replicas: DEFAULT_HASH_REPLICAS,
            hash_fn: Box::new(calculate_hash)
        }
    }
}

impl ConsistentHashBuilder {
    pub fn build(self) -> ConsistentHash {
        if self.replicas < 1 {
            panic!("nmsl");
        }
        ConsistentHash {
            replicas: self.replicas,
            hash_fn: self.hash_fn,
            hash_keys: Vec::new(), 
            hash_map: HashMap::new()
        }
    }
}

fn calculate_hash(t: &[u8]) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}