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

    /// Check if this string starts with another string.
    ///
    /// Returns `true` if the given pattern matches a prefix of this string slice.
    /// Returns `false` if it does not.
    pub fn startsWith(&self, pattern: OfinString) -> bool {
        self.inner.starts_with(&String::from(pattern))
    }

    /// Check if this string ends with another string.
    ///
    /// Returns `true` if the given pattern matches a suffix of this string slice.
    /// Returns `false` if it does not.
    pub fn endsWith(&self, pattern: OfinString) -> bool {
        self.inner.ends_with(&String::from(pattern))
    }

    /// Convert this string to uppercase
    pub fn toUpper(&self) -> OfinString {
        OfinString::new(self.inner.to_uppercase())
    }

    /// Convert this string to lowercase
    pub fn toLower(&self) -> OfinString {
        OfinString::new(self.inner.to_lowercase())
    }
}

impl<S: AsRef<str>> From<S> for OfinString {
    fn from(string: S) -> Self {
        OfinString::new(string.as_ref().to_string())
    }
}

impl From<OfinString> for String {
    fn from(string: OfinString) -> String {
        string.inner
    }
}