/// A string
pub struct OfinString {
    inner: String,
}

#[allow(non_snake_case)]
impl OfinString {
    /// Create a new string from a rust string 
    pub fn new<S: AsRef<str>>(string: S) -> Self {
        OfinString {
            inner: string.as_ref().to_string(),
        }
    }

    /// Convert this string into a rust string
    fn into_string(&self) -> &String {
        &self.inner
    }

    /// Check if a string starts with another string.
    ///
    /// Returns `true` if the given pattern matches a prefix of this string slice.
    /// Returns `false` if it does not.
    pub fn startsWith(&self, pattern: OfinString) -> bool {
        self.inner.starts_with(pattern.into_string())
    }

    /// Check if a string ends with another string.
    ///
    /// Returns `true` if the given pattern matches a suffix of this string slice.
    /// Returns `false` if it does not.
    pub fn endsWith(&self, pattern: OfinString) -> bool {
        self.inner.ends_with(pattern.into_string())
    }
}
