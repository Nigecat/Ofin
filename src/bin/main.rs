use ofin::util;
use std::{fs, process};
mod cli;
use cli::Cli;

#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init our logger
    pretty_env_logger::init();

    // Load our cli arguments
    let cli: Cli = Cli::read()?;

    let script = fs::read_to_string(&cli.script)?;
    let res = ofin::execute(script);
    if let Err(err) = res {
        eprintln!("{}", err);
        process::exit(1);
    }

    Ok(())
}
