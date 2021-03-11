mod error;
mod program;
mod token;
pub use error::OfinError;
pub use program::Program;

#[macro_use]
extern crate tracing;
