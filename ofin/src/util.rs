use std::ffi::OsStr;
use std::fs;
use std::path::Path;

/// Check if a binary is in the PATH
pub fn in_path<T: AsRef<OsStr>>(binary_name: T) -> bool {
    which::which(binary_name).is_ok()
}

/// Check if the given file path exists
pub fn path_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).is_ok()
}
