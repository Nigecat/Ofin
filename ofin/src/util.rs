use std::ffi::OsStr;

/// Check if a binary is in the PATH
pub fn in_path<T: AsRef<OsStr>>(binary_name: T) -> bool {
    which::which(binary_name).is_ok()
}
