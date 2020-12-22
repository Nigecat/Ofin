use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

/// An int
///
/// This is named `Int` as opposed to `OfinInt` to match the documentation on the string trait bindings
pub struct Int {
    inner: isize,
}

impl Int {
    /// Create a new int from a rust isize
    pub fn new(int: isize) -> Self {
        Int { inner: int }
    }
}

impl From<isize> for Int {
    fn from(int: isize) -> Self {
        Int::new(int)
    }
}

impl From<Int> for isize {
    fn from(int: Int) -> isize {
        int.inner
    }
}

impl From<Int> for usize {
    fn from(int: Int) -> usize {
        usize::try_from(int.inner).expect("Expected positive integer, got negative integer")
    }
}

impl From<Int> for u64 {
    fn from(int: Int) -> u64 {
        u64::try_from(int.inner).expect("Expected positive integer, got negative integer")
    }
}

impl Deref for Int {
    type Target = isize;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}
