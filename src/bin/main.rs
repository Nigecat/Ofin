use include_dir::{include_dir, Dir};
use ofin::util;
use std::io::prelude::*;
use std::{env, fs, process};
mod cli;
use cli::Cli;

// Include our static data
// This imports a single variable:
// * `RUSTC` - The binary data of the rust compiler this application was compiled with
include!(env!("STATIC_LOCATION"));

// Include our dependencies for the standard library
const DEPENDENCIES: Dir = include_dir!("ofin-std/target/release/deps");

#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init our logger
    pretty_env_logger::init();

    // Load our cli arguments
    let cli: Cli = Cli::read()?;

    // Get the directory that this binary is contained in
    let dir = util::executable_dir();
    // Extract the rust compiler if needed
    if !util::path_exists(dir.join("rustc.exe")) {
        let mut file = fs::File::create(dir.join("rustc.exe"))?;
        file.write_all(RUSTC)?;
    }
    // Extract our dependencies if needed
    if !util::path_exists(dir.join("lib")) {
        fs::create_dir(dir.join("lib"))?;
        for dependency in DEPENDENCIES.files() {
            let mut file = fs::File::create(dir.join("lib").join(dependency.path()))?;
            file.write_all(dependency.contents())?;
        }
    }

    let script = fs::read_to_string(&cli.script)?;
    let res = ofin::execute(script);
    if let Err(err) = res {
        eprintln!("{}", err);
        process::exit(1);
    }

    Ok(())
}
