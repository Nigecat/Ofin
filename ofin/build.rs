use std::env;
use std::path::Path;
use which::which;

fn main() {
    let rustc = which("rustc").unwrap();
    println!("cargo:rustc-env=RUSTC_LOCATION={}", rustc.to_str().unwrap());

    let profile = &env::var("PROFILE").unwrap();
    if Path::new("libofin_std.rlib").exists() {
        println!("cargo:rustc-env=STDLIB_LOCATION=libofin_std.rlib");
    } else {
        println!(
            "cargo:rustc-env=STDLIB_LOCATION=../../target/{}/libofin_std.rlib",
            profile
        );
    }
}
