#![allow(non_snake_case)]

mod bool;
mod int;
mod string;
pub use crate::bool::Bool;
pub use crate::string::String;
pub use int::Int;

#[doc(hidden)]
pub type OfinString = crate::string::String;

#[doc(hidden)]
pub mod prelude;
