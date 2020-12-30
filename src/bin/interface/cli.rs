use clap::{crate_authors, crate_version, Clap};
use ofin::util;

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Cli {
    /// The script to run
    pub script: String,
    /// Clear the internal cache prior to running this script
    #[clap(long)]
    clear_cache: bool,
    /// Clear the internal standard library cache prior to running this script
    #[clap(long)]
    clear_lib: bool,
}

impl Cli {
    pub fn read() -> Result<Self, String> {
        let cli = Cli::parse();

        // Ensure the script we were passed exists
        if !util::path_exists(&cli.script) {
            return Err(format!("{}: file not found", cli.script));
        }

        // Remove the cache if we need to
        if cli.clear_cache && util::path_exists(util::executable_dir().join("cache")) {
            std::fs::remove_dir_all(util::executable_dir().join("cache"))
                .expect("Unable to remove cache directory");
        }

        // Remove our standard library if we need to
        if cli.clear_lib && util::path_exists(util::executable_dir().join("lib")) {
            std::fs::remove_dir_all(util::executable_dir().join("lib"))
                .expect("Unable to remove standard lbirary directory");
        }

        Ok(cli)
    }
}