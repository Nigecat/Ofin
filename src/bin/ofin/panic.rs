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

    let msg = format!(
        "
An internal panic has occured, this is a bug.
Please make a bug report at {repo}/issues/new and include the following snippet:

---

{name} v{version} built from {hash}

{info}
trace{location}:
{trace}
    ",
        repo = env!("CARGO_PKG_REPOSITORY"),
        name = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        hash = env!("BUILD_GIT_HASH"), // Note that this is set in build.rs
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
