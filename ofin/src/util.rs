use std::path::{Path, PathBuf};
use std::{env, fs};

/// Check if the given file path exists
pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).is_ok()
}

/// Get the directory of the current executable
pub fn executable_dir() -> PathBuf {
    let mut self_dir = env::current_exe().unwrap();
    self_dir.pop();
    self_dir
}
