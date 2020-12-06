use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

/// An md5 hash
type Hash = String;

/// Calculate the md5 hash of data
fn calculate_hash<T: AsRef<[u8]>>(data: T) -> Hash {
    let digest = md5::compute(data);
    format!("{:x}", digest)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    inner: HashMap<Hash, String>,
}

impl Cache {
    /// Create a new cache or load it from disk
    ///
    /// This will automatically write all changes to `cache.bin`.
    pub fn new() -> Self {
        // Check if cache.bin exists
        if fs::metadata("cache.bin").is_ok() {
            fs::read("cache.bin").unwrap().into()
        } else {
            fs::File::create("cache.bin").unwrap();
            Cache {
                inner: HashMap::new(),
            }
        }
    }
}

impl From<Cache> for Vec<u8> {
    fn from(cache: Cache) -> Self {
        bincode::serialize(&cache).unwrap()
    }
}

impl From<Vec<u8>> for Cache {
    fn from(data: Vec<u8>) -> Self {
        bincode::deserialize(&data[..]).unwrap()
    }
}
