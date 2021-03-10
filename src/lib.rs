mod error;
mod program;
pub use error::OfinError;
pub use program::Program;

#[macro_use]
extern crate tracing;
