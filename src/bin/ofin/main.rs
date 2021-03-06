mod cli;
use cli::Cli;
use std::{fs, panic};
use structopt::StructOpt;
use tracing::error;

fn main() {
    tracing_subscriber::fmt::init();

    panic::set_hook(Box::new(|panic_info| {
        let info = panic_info.payload().downcast_ref::<&str>().unwrap_or(&"");
        let location = panic_info
            .location()
            .map(|location| format!("from {}:{}", location.line(), location.file()))
            .unwrap_or_default();

        let repo = env!("CARGO_PKG_REPOSITORY");
        let version = env!("CARGO_PKG_VERSION");
        let authors = env!("CARGO_PKG_AUTHORS");
        let date = ofin::build::BUILT_TIME_UTC;
        let hash = ofin::build::GIT_COMMIT_HASH.unwrap_or("");
        let target = ofin::build::TARGET;
        let rustc = ofin::build::RUSTC_VERSION;

        let msg = format!(
            "
            An internal panic has occured, this is a bug.
            Please make a bug report at {repo} and include the following snippet:

            v{version}
            created by {authors} and 
            built on {date}
            with hash {hash}
            for target {target}
            using {rustc}.

            trace{location}:
                {info}
        ",
            repo = repo,
            version = version,
            authors = authors,
            date = date,
            hash = hash,
            target = target,
            rustc = rustc,
            info = info,
            location = location,
        );

        if tracing::dispatcher::has_been_set() {
            error!("{}", msg);
        } else {
            println!("error: {}", msg);
        }
    }));

    let args = Cli::from_args();

    if let Ok(contents) = fs::read_to_string(&args.input) {
        if let Err(e) = ofin::run(contents) {
            e.report();
        }
    } else {
        error!("file not found: {:?}", args.input);
    }
}
