use std::env;
use std::ffi::OsStr;
use std::process::Command;

fn main() {
    // Fetch the current git hash by running `git rev-parse HEAD`
    let hash = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();

    println!(
        "cargo:rustc-env=BUILD_GIT_HASH={}",
        String::from_utf8_lossy(&hash.stdout)
    );
}
