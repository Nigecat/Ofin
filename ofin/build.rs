use std::path::Path;
use std::{env, fs};
use which::which;

fn main() {
    let profile = &env::var("PROFILE").unwrap();

    // Get the location of rustc
    let rustc = which("rustc").unwrap();
    println!("cargo:rustc-env=RUSTC_LOCATION={}", rustc.to_str().unwrap());

    // Get the standard library rlib location
    if Path::new("libofin_std.rlib").exists() {
        println!("cargo:rustc-env=STDLIB_LOCATION=libofin_std.rlib");
    } else {
        println!(
            "cargo:rustc-env=STDLIB_LOCATION={}",
            fs::canonicalize(format!("../target/{}/libofin_std.rlib", profile))
                .unwrap()
                .to_str()
                .unwrap(),
        );
    }
}
