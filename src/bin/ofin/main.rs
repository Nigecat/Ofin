mod cli;
use cli::Cli;
use std::fs;
use structopt::StructOpt;
use tracing::error;

fn main() {
    tracing_subscriber::fmt::init();

    let args = Cli::from_args();

    if let Ok(contents) = fs::read_to_string(&args.input) {
        if let Err(e) = ofin::run(contents) {
            e.report();
        }
    } else {
        error!("file not found: {:?}", args.input);
    }
}
