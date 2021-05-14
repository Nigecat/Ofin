mod cli;
mod panic;
use cli::Cli;

fn main() {
    panic::set_handler();

    let cli = Cli::from_args();

    let res = ofin::run(cli.file);
    match res {
        Ok(()) => println!("Program exited with error code 0"),
        Err(e) => println!("{}", e),
    }
}
