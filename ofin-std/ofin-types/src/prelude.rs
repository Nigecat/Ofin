pub use super::Bool as __bool;
pub use super::Int as __int;
pub use super::OfinString as __string;
use super::*;

pub fn __new_string<S: AsRef<str>>(string: S) -> OfinString {
    OfinString::new(string)
}

pub fn __new_int(int: isize) -> Int {
    Int::new(int)
}

pub fn __new_bool(bl: bool) -> Bool {
    Bool::new(bl)
}