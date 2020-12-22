#![allow(non_snake_case)]

mod int;
mod string;
mod bool;
pub use int::Int;
pub use crate::string::String;
pub use crate::bool::Bool;
pub type OfinString = crate::string::String;