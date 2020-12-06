#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
mod error;
mod transpiler;
pub use error::OfinError;
pub mod util;

/// Execute a script
pub fn execute(mut script: String) -> Result<(), OfinError> {
    trace!("Transpiling source script...");

    // Convert our ofin script into rust code
    script = transpiler::transpile(script);
    println!("{:#?}:", script);

    Ok(())
}
