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
use std::env::current_dir;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

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

    // Check if this script is not in the cache
    if !Cache::has(&script) {
        // Write the transpiled script to a temporary location so we can pass it to rustc
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", &script).unwrap();

        let command = Command::new("rustc")
            .args(&[
                &file.path().to_str().unwrap(),
                "-o",
                &Cache::get(&script).to_str().unwrap(),
                "--crate-name",
                "ofin",
            ])
            .output()
            .expect("failed to invoke rustc");
        let err = std::str::from_utf8(&command.stderr).unwrap().trim();
        if !err.is_empty() {
            error!("{}", err);
        }
    }

    let path = Cache::get(script);
    debug!("Running executable: {:?}", path);

    Command::new(current_dir().unwrap().join(path))
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("unable to start program");

    Ok(())
}
