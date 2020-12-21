use std::io::prelude::*;
use std::{env, fs, process};
mod cli;
#[allow(dead_code)]
mod util;
use cli::Cli;

// Include our static data
// This imports two variables:
// * `RUSTC` - The binary data of the rust compiler this application was compiled with
// * `STDLIB` - The binary data of our built standard library
include!(env!("STATIC_LOCATION"));

#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init our logger
    pretty_env_logger::init();

    // Load our cli arguments
    let cli: Cli = Cli::read()?;

    // Get the directory that this binary is contained in
    let dir = util::executable_dir();
    // Extract the rust compiler and our stdlib
    if !util::path_exists(dir.join("rustc.exe")) {
        let mut file = fs::File::create(dir.join("rustc.exe"))?;
        file.write_all(RUSTC)?;
    }
    if !util::path_exists(dir.join("stdlib.rlib")) {
        let mut file = fs::File::create(dir.join("stdlib.rlib"))?;
        file.write_all(STDLIB)?;
    }

    let script = fs::read_to_string(&cli.script)?;
    let res = ofin::execute(script);
    if let Err(err) = res {
        eprintln!("{}", err);
        process::exit(1);
    }

    Ok(())
}
