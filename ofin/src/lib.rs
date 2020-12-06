#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
mod cache;
mod error;
mod transpiler;
pub use error::OfinError;
pub mod util;
use cache::Cache;

lazy_static! {
    static ref CACHE: Cache = Cache::new();
}

/// Execute a script
pub fn execute(mut script: String) -> Result<(), OfinError> {
    // Make sure we have the rust compiler installed
    if !util::in_path("rustc") {
        return Err(OfinError::RustCNotFound);
    }

    info!("Transpiling source script...");

    // Convert our ofin script into rust code
    script = transpiler::transpile(script);

    Ok(())
}
