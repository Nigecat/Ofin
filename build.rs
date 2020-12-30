use glob::glob;
use std::process::Command;
use std::{env, fs};
use which::which;

fn main() {
    let rustc = which("rustc")
        .expect("Unable to locate the rust compiler, please ensure rustc is on your PATH");
    println!("cargo:rustc-env=RUSTC_LOCATION={}", rustc.to_str().unwrap());

    // Rerun if our standard library has been modified
    for entry in glob("ofin-std/**/*.rs").unwrap() {
        if let Ok(path) = entry {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    // [Re]build our standard library so we can embed it
    env::set_current_dir("ofin-std").unwrap();
    let _ = fs::remove_dir_all("target/release/deps");
    Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .expect("stdlib build failed");

    println!("cargo:rustc-env=STDLIB_LOCATION=ofin-std/target/release/deps");
}
