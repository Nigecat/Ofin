use crate::build::{PKG_DESCRIPTION, PKG_NAME};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = PKG_NAME, about = PKG_DESCRIPTION)]
pub struct Cli {
    /// Input file
    #[structopt(parse(from_os_str))]
    pub input: PathBuf,
}

impl Cli {
    pub fn from_args() -> Self {
        StructOpt::from_args()
    }
}
