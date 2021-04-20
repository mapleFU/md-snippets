use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("set data duplicate")]
    Duplicate,

    #[error("other error")]
    Other,
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait Storage {
    fn set_if_absent(&self, long_url: &str, short_url: &str) -> Result<Option<()>>;
    fn get_content(&self, short_url: &str) -> Result<Option<String>>;
}

#[derive(Default)]
pub struct HashMapStorage {
    hash_map: Arc<Mutex<HashMap<String, String>>>,
}

impl Storage for HashMapStorage {
    fn set_if_absent(&self, long_url: &str, short_url: &str) -> Result<Option<()>> {
        let mut hash_map = self.hash_map.lock().unwrap();
        let opt = hash_map.get(short_url);
        if !opt.is_none() {
            return Ok(None);
        }
        hash_map.insert(short_url.to_owned(), long_url.to_owned());
        println!("put {:?} -> {:?}", short_url, long_url);
        Ok(Some(()))
    }

    fn get_content(&self, short_url: &str) -> Result<Option<String>> {
        println!("get {:?} ", short_url);
        Ok(self.hash_map.lock().unwrap().get(short_url).cloned())
    }
}
