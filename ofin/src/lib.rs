#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
mod cache;
mod error;
mod lexer;
mod transpiler;
pub use error::OfinError;
mod util;
use cache::Cache;
use std::io::Write;
use std::process::{Command, Stdio};
use tempfile::NamedTempFile;

/// Execute a script
pub fn execute(script: String) -> Result<(), OfinError> {
    Cache::init();

    let current_dir = util::executable_dir();

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
            current_dir.to_str().unwrap(),
            "-L", // FIXME: Make this a proper dynamic link to the dependencies
            "target/deps",
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
