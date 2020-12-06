//! Standard library string manipulation functions

/// Check if a string starts with a pattern
/// 
/// Returns `true` if the given pattern matches the start of the given string.  
/// Returns `false` if it does not.
pub fn starts_with(string: String, pattern: String) -> bool {
    string.starts_with(&pattern)
}

/// Check if a string ends with a pattern
/// 
/// Returns `true` if the given pattern matches the end of the given string.  
/// Returns `false` if it does not.
pub fn ends_with(string: String, pattern: String) -> bool {
    string.ends_with(&pattern)
}