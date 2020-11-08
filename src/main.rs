#[macro_use]
extern crate log;
#[macro_use]
mod error;
mod application;
mod cli;
mod constant_table;
mod symbol_table;
mod symbols;
mod tokenizer;
mod tokens;
use application::Application;
use std::{env, fs};

fn _main() -> Result<(), error::Error> {
    let args = cli::CLI::parse()?;

    // Make sure the source file exists
    if fs::metadata(&args.path).is_err() {
        return Err(error!("{}: no such file or directory", args.path));
    }

    // Read the source file
    let source = fs::read_to_string(&args.path).unwrap();

    let application = Application::new();
    let tokenstream = tokenizer::tokenize(&source, &application);
    println!("{:#?}", tokenstream);

    Ok(())
}

fn main() {
    // Default our debug logging to `info`, since we are still in heavy developement
    env::set_var(
        "RUST_LOG",
        env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    );
    pretty_env_logger::init();

    // Handle all our errors by printing them
    let res = _main();
    if let Err(e) = res {
        println!("{}", e);
    }
}
