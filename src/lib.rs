mod error;
pub use error::OfinError;

use std::path::PathBuf;

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate lazy_static;

/// Run a script
///
///  * `script` - The file path of the script to run, this should ideally be passed through [fs::canonicalize](std::fs::canonicalize) beforehand
pub fn run(script: PathBuf) -> Result<(), error::OfinError> {
    Ok(())
}
