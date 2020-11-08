use std::fmt;
mod int;
mod string;
pub use int::SInt;
pub use string::SString;

pub trait Symbol: fmt::Debug {
    /// Attemp to convert a string into this symbol
    fn new(from: &str) -> Option<Self>
    where
        Self: Sized;
}
