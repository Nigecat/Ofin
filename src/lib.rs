mod error;
mod program;
mod statements;
mod token;
mod utils;
mod value;

pub use error::{Error, SyntaxError};
pub use ofin_std as std;
pub use program::Program;

// #[macro_use]
// extern crate tracing;
