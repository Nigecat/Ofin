mod directive;
mod error;
mod instruction;
mod source;
mod token;
mod vm;

pub use error::Error;
pub use ofin_std as std;
pub use source::Source;

// #[macro_use]
// extern crate tracing;

/// Run an ofin program from the supplied source.
pub fn run<T: Source>(source: T) -> Result<(), Error> {
    Ok(())
}
