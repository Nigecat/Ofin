#[macro_use]
extern crate log;
#[macro_use]
extern crate enum_display_derive;
mod application;
mod cli;
mod constant_table;
mod errors;
mod symbol_table;
mod symbols;
mod tokenizer;
mod tokens;
use application::Application;
use std::{env, fs};

fn _main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::CLI::parse()?;

    // Read the source file
    let source = fs::read_to_string(&args.path).unwrap();

    let mut application = Application::new();
    let tokenstream = tokenizer::tokenize(&source, &mut application)?;
    println!("{:#?}", tokenstream);

    Ok(())
}

fn main() {
    // Default our debug logging to `warn`
    env::set_var(
        "RUST_LOG",
        env::var("RUST_LOG").unwrap_or_else(|_| "warn".to_string()),
    );
    pretty_env_logger::init();

    // Handle all our errors by printing them
    let res = _main();
    if let Err(e) = res {
        log::error!("{}", e);
    }
}
