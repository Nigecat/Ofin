mod cli;
mod panic;
use cli::Cli;
use ofin::Program;
use tracing::error;

fn main() {
    panic::set_handler();

    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let cli = Cli::from_args();

    let result = match Program::new(cli.file) {
        Ok(mut program) => program.run(),
        Err(e) => Err(e),
    };

    match result {
        Ok(_) => (),
        Err(e) => error!("{}", e),
    }
}
