use crate::util::path_exists;
use std::fs;
use std::path::{Path, PathBuf};

/// The relative directory to store cache artifacts in
static CACHE_DIR: &str = "cache";

/// An md5 hash
type Hash = String;

/// Calculate the md5 hash of data
fn calculate_hash<T: AsRef<[u8]>>(data: T) -> Hash {
    let digest = md5::compute(data);
    format!("{:x}", digest)
}

pub struct Cache {}

impl Cache {
    /// Init the cache, this should be called before attempting to access anything
    pub fn init() -> Self {
        fs::create_dir_all("cache").unwrap();
        Cache {}
    }

    /// Check if some data is in the cache
    pub fn has<T: AsRef<[u8]>>(data: T) -> bool {
        path_exists(&Cache::get(data))
    }

    /// Get the file path to some cached data.
    ///
    /// This **does not** guarantee that the path exists,
    /// that should be checked prior using the [get](#method.get) method.
    pub fn get<T: AsRef<[u8]>>(data: T) -> PathBuf {
        Path::new(CACHE_DIR).join(calculate_hash(data))
    }

    /*
    /// Cache some data.
    ///
    /// Returns a path to the cached data.
    pub fn cache<T: AsRef<[u8]>>(data: T) -> PathBuf {
        let hash = calculate_hash(&data);
        let path = Cache::get(hash);
        let mut file = File::create(&path).unwrap();
        file.write_all(data.as_ref()).unwrap();

        path
    }
    */
}
