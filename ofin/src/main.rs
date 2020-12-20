use std::io::prelude::*;
use std::{env, fs, process};
#[allow(dead_code)]
mod util;

static RUSTC: &[u8] = include_bytes!(env!("RUSTC_LOCATION"));
static STDLIB: &[u8] = include_bytes!(env!("STDLIB_LOCATION"));

#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init our logger
    pretty_env_logger::init();

    // Get the directory that this binary is contained in
    let dir = util::executable_dir();
    // Extract the rust compiler, our stdlib, and our dependencies
    if !util::path_exists(dir.join("rustc.exe")) {
        let mut file = fs::File::create(dir.join("rustc.exe")).unwrap();
        file.write_all(RUSTC).unwrap();
    }
    if !util::path_exists(dir.join("stdlib.rlib")) {
        let mut file = fs::File::create(dir.join("stdlib.rlib")).unwrap();
        file.write_all(STDLIB).unwrap();
    }

    if let Some(script_path) = env::args().nth(1) {
        if let Ok(script) = fs::read_to_string(&script_path) {
            let res = ofin::execute(script);
            if let Err(err) = res {
                eprintln!("{}", err);
                process::exit(1);
            } else {
                process::exit(0);
            }
        } else {
            eprintln!("{}: file not found", script_path);
            process::exit(1);
        }
    } else {
        eprintln!("usage: ofin [script]");
        process::exit(1);
    }
}
