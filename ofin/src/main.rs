use std::{env, fs, process};

#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init our logger
    pretty_env_logger::init();

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
