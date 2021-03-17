mod build;
mod cli;
use cli::Cli;

fn main() {
    tracing_subscriber::fmt::init();

    let _cli = Cli::from_args();
}
