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
        ];
        debug!("Invoking `rustc {}`", args.join(" "));
        let command = Command::new("rustc")
            .args(args)
            .output()
            .expect("failed to invoke rustc");
        let err = std::str::from_utf8(&command.stderr).unwrap().trim();
        if !err.is_empty() {
            error!("{}", err);
        }
    }

    let path = Cache::get(script);
    debug!("Running executable: {:?}", path);

    Command::new(current_dir.join(path))
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .spawn()
        .expect("unable to start program");

    Ok(())
}
