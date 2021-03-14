//! The ofin standard library.

#![warn(missing_docs)]

pub mod time;
pub mod filesystem;
pub mod random;
pub mod process;
pub mod prelude;

#[doc(hidden)]
pub mod internal;