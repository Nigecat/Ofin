use std::path::PathBuf;
use structopt::StructOpt;

const NAME: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Debug, StructOpt)]
#[structopt(name = NAME, about = DESCRIPTION, author = AUTHOR)]
pub struct Cli {
    /// Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}
