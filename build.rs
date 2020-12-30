use std::path::Path;
use std::process::Command;
use std::{env, fs};
use which::which;
use glob::glob;

fn main() {
    // Rerun if our standard library has been modified
    for entry in glob("ofin-std/**/*.rs").unwrap() {
        if let Ok(path) = entry {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    // Build our standard library so the include_dir crate can access it
    env::set_current_dir("ofin-std").unwrap();
    let _ = Command::new("cargo")
        .args(&["build", "--release"])
        .output();

    // Bundle the rust compiler for linking our standard library
    let rustc = fs::canonicalize(which("rustc").unwrap()).unwrap();
    let rustc = rustc.to_str().unwrap();
    let rustc = format!(
        "static RUSTC: &[u8] = include_bytes!({});",
        snailquote::escape(rustc),
    );

    let data = format!("{}", rustc);

    let out_dir = &env::var("OUT_DIR").unwrap();
    let path = Path::new(out_dir).join("static.rs");

    fs::write(&path, data).expect("unable to write static data to file");

    println!("cargo:rustc-env=STATIC_LOCATION={}", path.to_str().unwrap());
}
