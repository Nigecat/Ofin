#[macro_use]
extern crate log;
mod error;
pub use error::OfinError;
pub mod util;

/// Execute a script
pub fn execute(script: String) -> Result<(), OfinError> {
    trace!("Executing script:\n{}", script);

    Ok(())
}