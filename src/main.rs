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
use std::fs;

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
    // Handle all our errors by printing them
    let res = _main();
    if let Err(e) = res {
        println!("{}", e);
    }
}
