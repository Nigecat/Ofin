//! The ofin standard library.

#![warn(missing_docs)]

pub mod filesystem;
pub mod prelude;
pub mod process;
pub mod random;
pub mod time;

mod types;
pub use types::*;
