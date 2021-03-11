mod build;
mod cli;
use cli::Cli;
use std::panic;
use structopt::StructOpt;

fn panic_handler(panic_info: &panic::PanicInfo) {
    let info = panic_info.payload().downcast_ref::<&str>().unwrap_or(&"");
    let location = panic_info
        .location()
        .map(|location| format!(" from {}:{}", location.file(), location.line()))
        .unwrap_or_default();

    let repo = env!("CARGO_PKG_REPOSITORY");
    let version = env!("CARGO_PKG_VERSION");
    let date = build::BUILT_TIME_UTC;
    let hash = build::GIT_COMMIT_HASH.unwrap_or("");
    let target = build::TARGET;
    let rustc = build::RUSTC_VERSION;

    let msg = format!(
        "
An internal panic has occured, this is a bug.
Please make a bug report at {repo} and include the following snippet:

v{version}
built on {date}
with hash {hash}
for target {target}
using {rustc}.

{info}

trace{location}:
{trace}
    ",
        repo = repo,
        version = version,
        date = date,
        hash = hash,
        target = target,
        rustc = rustc,
        info = info,
        trace = panic_info,
        location = location,
    );

    if tracing::dispatcher::has_been_set() {
        tracing::error!("{}", msg);
    } else {
        println!("error: {}", msg);
    }
}

fn main() {
    panic::set_hook(Box::new(panic_handler));

    tracing_subscriber::fmt::init();

    let args = Cli::from_args();

    let program = ofin::Program::parse_from_file(args.input);

    let result = match program {
        Ok(mut program) => program.run(),
        Err(e) => Err(e),
    };

    if let Err(e) = result {
        e.report();
    }
}
