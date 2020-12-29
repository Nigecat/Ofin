use std::path::Path;
use std::{env, fs};
use which::which;

fn main() {
    let rustc = format!(
        "static RUSTC: &[u8] = include_bytes!({});",
        snailquote::escape(
            fs::canonicalize(which("rustc").unwrap())
                .unwrap()
                .to_str()
                .unwrap()
        ),
    );

    let data = format!("{}", rustc);

    let out_dir = &env::var("OUT_DIR").unwrap();
    let path = Path::new(out_dir).join("static.rs");

    fs::write(&path, data).expect("unable to write static data to file");

    println!("cargo:rustc-env=STATIC_LOCATION={}", path.to_str().unwrap());
}
