use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

/// An int
pub struct OfinInt {
    inner: isize,
}

impl OfinInt {
    /// Create a new int from a rust isize
    pub fn new(int: isize) -> Self {
        OfinInt { inner: int }
    }
}

impl From<isize> for OfinInt {
    fn from(int: isize) -> Self {
        OfinInt::new(int)
    }
}

impl From<OfinInt> for isize {
    fn from(int: OfinInt) -> isize {
        int.inner
    }
}

impl From<OfinInt> for usize {
    fn from(int: OfinInt) -> usize {
        usize::try_from(int.inner).expect("Expected positive integer, got negative integer")
    }
}

impl From<OfinInt> for u64 {
    fn from(int: OfinInt) -> u64 {
        u64::try_from(int.inner).expect("Expected positive integer, got negative integer")
    }
}

impl Deref for OfinInt {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl fmt::Display for OfinInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
