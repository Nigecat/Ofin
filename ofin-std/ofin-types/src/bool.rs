use std::fmt;
use std::ops::Deref;

/// A boolean
pub struct Bool {
    inner: bool,
}

impl Bool {
    /// Create a new bool from a rust bool
    pub fn new(bl: bool) -> Self {
        Bool { inner: bl }
    }
}

impl From<Bool> for bool {
    fn from(bl: Bool) -> bool {
        bl.inner
    }
}

impl From<bool> for Bool {
    fn from(bl: bool) -> Bool {
        Bool::new(bl)
    }
}

impl Deref for Bool {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
