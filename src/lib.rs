#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
use tempfile::NamedTempFile;

mod cache;
mod error;
mod lexer;
mod transpiler;
use cache::Cache;
pub mod util;
pub use error::OfinError;

use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Stub function to prevent the rust compiler from erroring
#[cfg(debug_assertions)]
fn init(_: &PathBuf) -> Result<(), std::io::Error> {
    Ok(())
}

/// Run preprocessing steps for release mode
#[cfg(not(debug_assertions))]
fn init(dir: &PathBuf) -> Result<(), std::io::Error> {
    // ----- The following code should only run in a release build -----
    use include_dir::{include_dir, Dir};
    use std::fs;

    let rustc: &[u8] = include_bytes!(env!("RUSTC_LOCATION"));
    let stdlib: Dir = include_dir!("ofin-std/target/release/deps");

    // Extract the rust compiler if needed
    if !util::path_exists(dir.join("rustc.exe")) {
        debug!("Extracting rust compiler...");
        let mut file = fs::File::create(dir.join("rustc.exe"))?;
        file.write_all(rustc)?;
    }

    // Extract our standard library if needed
    if !util::path_exists(dir.join("lib")) {
        debug!("Extracting standard library...");
        fs::create_dir(dir.join("lib"))?;
        for file in stdlib.files() {
            let mut f = fs::File::create(dir.join("lib").join(file.path()))?;
            f.write_all(file.contents())?;
        }
    }

    // Point the standard library location to the local directory
    env::set_var("STDLIB_LOCATION", dir.join("lib"));

    Ok(())
}

/// Execute a script
pub fn execute(script: String) -> Result<(), OfinError> {
    Cache::init();

    let current_dir = util::executable_dir();

    init(&current_dir)?;

    // Check if this script is not in the cache
    if !Cache::has(&script) {
        let tokens = lexer::lex(&script)?;
        debug!("Got tokens: {:?}", tokens);

        // Convert our ofin script into rust code
        let rc = transpiler::transpile(tokens)?;

        // Write the transpiled script to a temporary location so we can pass it to rustc
        let mut file = NamedTempFile::new().unwrap();
        write!(file, "{}", &rc).unwrap();

        let path = Cache::get(&script);
        let args = &[
            &file.path().to_str().unwrap(),
            "-o",
            &path.to_str().unwrap(),
            "--crate-name",
            "ofin",
            "-L",
            &env::var("STDLIB_LOCATION").unwrap(),
        ];
        debug!("Invoking `rustc {}`", args.join(" "));
        let command = Command::new("rustc")
            .args(args)
            .output()
            .expect("failed to invoke rustc");

        let err = std::str::from_utf8(&command.stderr).unwrap().trim();
        if !err.is_empty() {
            trace!("{}", err);

            let mut errors = error::ErrorStream::new();

            for error in &OfinError::from_rustc(err) {
                if error.0 > 0 {
                    errors.push(OfinError::SyntaxErrorV2 {
                        row: error.0 as usize,
                        ctx: error.1.clone(),
                        code: script.split('\n').collect::<Vec<_>>()[error.0 as usize - 1]
                            .to_string(),
                    });
                } else {
                    // This must be an internal error if we managed to get an underflow
                    return Err(OfinError::InternalError(err.to_string()));
                }
            }

            return Err(OfinError::Multi { errors });
        }
    }

    let path = Cache::get(script);
    debug!("Running executable: {:?}", path);

    Command::new(current_dir.join(path))
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("unable to start program")
        .wait()
        .expect("unable to start program");

    Ok(())
}
