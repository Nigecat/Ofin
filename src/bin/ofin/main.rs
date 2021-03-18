mod build;
mod cli;
mod panic;
use cli::Cli;
use ofin::Program;
use tracing::error;

fn main() {
    panic::set_handler();

    tracing_subscriber::fmt::init();

    let cli = Cli::from_args();

    let program = Program::from_file(cli.file);

    match program {
        Ok(_) => (),
        Err(e) => error!("{}", e),
    }
}
