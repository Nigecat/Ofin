use crate::build;
use std::panic;

/// Set the custom panic handler
pub fn set_handler() {
    panic::set_hook(Box::new(panic_handler));
}

pub fn panic_handler(panic_info: &panic::PanicInfo) {
    let info = panic_info.payload().downcast_ref::<&str>().unwrap_or(&"");
    let location = panic_info
        .location()
        .map(|location| format!(" from {}:{}", location.file(), location.line()))
        .unwrap_or_default();

    let repo = build::PKG_REPOSITORY;
    let version = build::PKG_VERSION;
    let date = build::BUILT_TIME_UTC;
    let hash = build::GIT_COMMIT_HASH.unwrap_or("");
    let target = build::TARGET;
    let rustc = build::RUSTC_VERSION;

    let msg = format!(
        "
An internal panic has occured, this is a bug.
Please make a bug report at {repo} and include the following snippet:

---

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
