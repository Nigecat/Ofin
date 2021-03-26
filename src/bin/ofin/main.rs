mod build;
mod cli;
mod panic;
use cli::Cli;
use tracing::error;

fn main() {
    panic::set_handler();

    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let cli = Cli::from_args();

    let result = ofin::run(cli.file.into());

    match result {
        Ok(_) => (),
        Err(e) => error!("{}", e),
    }
}
