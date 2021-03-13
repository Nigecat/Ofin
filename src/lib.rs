mod error;
mod program;
mod token;
pub use error::Error;
pub use ofin_std::*;
pub use program::Program;

#[macro_use]
extern crate tracing;
