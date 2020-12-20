use std::ops::Deref;
use std::fmt;

/// A single character
pub struct OfinChar {
    inner: char,
}

impl OfinChar {
    /// Create a new char from a rust string
    pub fn new(ch: char) -> Self {
        OfinChar {
            inner: ch,
        }
    }
}

impl From<char> for OfinChar {
    fn from(ch: char) -> Self {
        OfinChar::new(ch)
    }
}

impl From<OfinChar> for char {
    fn from(ch: OfinChar) -> char {
        ch.inner
    }
}

impl Deref for OfinChar {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl fmt::Display for OfinChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
