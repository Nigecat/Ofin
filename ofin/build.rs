use std::path::Path;
use std::{env, fs};
use which::which;

fn main() {
    let profile = &env::var("PROFILE").unwrap();
    let out_dir = &env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(out_dir);

    let rustc = format!(
        "static RUSTC: &[u8] = include_bytes!({});",
        snailquote::escape(
            fs::canonicalize(which("rustc").unwrap())
                .unwrap()
                .to_str()
                .unwrap()
        ),
    );

    let stdlib = format!(
        "static STDLIB: &[u8] = include_bytes!({});",
        snailquote::escape(
            fs::canonicalize(
                env::current_dir()
                    .unwrap()
                    .join("..")
                    .join("target")
                    .join(profile)
                    .join("libofin_std.rlib")
            )
            .unwrap()
            .to_str()
            .unwrap()
        ),
    );

    let data = format!("{}\n{}", rustc, stdlib);
    let path = out_dir.join("static.rs");

    fs::write(&path, data).expect("unable to write static data to file");

    println!("cargo:rustc-env=STATIC_LOCATION={}", path.to_str().unwrap());
}
