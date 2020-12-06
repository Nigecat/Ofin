use std::{fs, env, process};

#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init our logger and default to a loglevel of `info`
    env::set_var("RUST_LOG", env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()));
    pretty_env_logger::init();

    // Ensure we have the rust compiler installed and available
    if !ofin::util::in_path("rustc") {
        eprintln!("rustc not detected,");
        if cfg!(windows) {
            eprintln!("please download 'https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe' to install rustc before running this program again");
        } else {
            eprintln!("please run 'curl https://sh.rustup.rs -sSf | sh' in your shell to install rustc before running this program again")
        }
        eprintln!("if the above method does not work, head over to the official rust website at https://www.rust-lang.org/ and install it from there");

        process::exit(1);
    }

    if let Some(script_path) = env::args().nth(1) {
        if let Ok(script) = fs::read_to_string(&script_path) {
            let res = ofin::execute(script);
            if let Err(err) = res {
                eprintln!("{}", err);
                process::exit(1);
            } else {
                process::exit(0);
            }
        } else {
            eprintln!("{}: file not found", script_path);
            process::exit(1);
        }
    } else {
        eprintln!("usage: ofin [script]");
        process::exit(1);
    }
}
