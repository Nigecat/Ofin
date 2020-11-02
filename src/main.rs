#[macro_use]
extern crate bitflags;
mod flags;
#[macro_use]
mod error;
mod cli;

fn _main() -> Result<(), error::Error> {
    let args = cli::CLI::parse()?;

    Ok(())
}

fn main() {
    // Handle all our errors by printing them
    let res = _main();
    if let Err(e) = res {
        println!("{}", e);
    }
}
