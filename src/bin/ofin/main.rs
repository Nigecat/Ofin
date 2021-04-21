mod cli;
mod panic;
use cli::Cli;
use ofin::Program;

fn main() {
    panic::set_handler();

    let cli = Cli::from_args();

    let result = match Program::new(cli.file) {
        Ok(mut program) => program.run(),
        Err(e) => Err(e),
    };

    match result {
        Ok(_) => (),
        Err(e) => println!("error: {}", e),
    }
}
