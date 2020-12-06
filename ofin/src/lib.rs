#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
mod error;
mod transpiler;
pub use error::OfinError;
pub mod cache;
pub mod util;
use cache::Cache;

/// Execute a script
pub fn execute(mut script: String) -> Result<(), OfinError> {
    Cache::init();

    // Make sure we have the rust compiler installed
    if !util::in_path("rustc") {
        return Err(OfinError::RustCNotFound);
    }

    info!("Transpiling source script...");

    // Convert our ofin script into rust code
    script = transpiler::transpile(script);

    // Check if this script is in the cache
    if Cache::has(&script) {
        let _path = Cache::get(script);
    // TODO: Execute script from path
    } else {
        // TODO: Build script and cache it
    }

    Ok(())
}
