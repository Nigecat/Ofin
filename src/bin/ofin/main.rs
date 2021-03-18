mod build;
mod cli;
mod panic;
use cli::Cli;
use ofin::Program;
use tracing::error;

fn main() {
    panic::set_handler();

    tracing_subscriber::fmt::init();

    let _cli = Cli::from_args();

    let program = Program::parse("using <time>;".to_string());

    match program {
        Ok(_) => (),
        Err(e) => error!("{}", e),
    }
}
