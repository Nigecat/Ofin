mod build;
mod cli;
mod panic;
use cli::Cli;

fn main() {
    panic::set_handler();

    tracing_subscriber::fmt::init();

    let _cli = Cli::from_args();

    assert_eq!(1, 2);
}
