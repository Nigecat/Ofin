use std::env;
use std::path::PathBuf;
use std::process::exit;

pub struct Cli {
    /// Input file
    pub file: PathBuf,
}

impl Cli {
    pub fn from_args() -> Self {
        let mut args: Vec<String> = env::args().collect();
        let program = args.remove(0);

        // Help info: -h | --help
        if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
            print!(
                "\
{name} {version}
{description}

USAGE:
    {program} <file>

FLAGS:
    -h, --help      Prints help information
    -V, --version   Prints version information

ARGS:
    <file>  Input file
                ",
                name = env!("CARGO_PKG_NAME"),
                version = env!("CARGO_PKG_VERSION"),
                description = env!("CARGO_PKG_DESCRIPTION"),
                program = program
            );
            exit(0);
        }

        // Version info: -V | --version
        if args.contains(&String::from("V")) {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            exit(0);
        }

        // Validate the remaining arguments (the only thing left should be the input file)
        if args.is_empty() {
            println!(
                "\
error: The following required arguments were not provided:
    <file>

USAGE:
    {program} <file>

Run with --help for more information
            ",
                program = program
            );
            exit(1);
        }

        // Check if we got too many arguments and whether we have any excess flags
        if args.len() > 1 || args.iter().any(|a| a.starts_with('-')) {
            println!(
                "\
error: Found argument '{arg}' which wasn't expected, or isn't valid in this context

USAGE:
    {program} <file>

For more information try --help
    ",
                arg = args
                    .get(1)
                    .unwrap_or_else(|| args.iter().find(|a| a.starts_with('-')).unwrap()),
                program = program,
            );
            exit(1);
        }

        Cli {
            file: PathBuf::from(args.remove(0)),
        }
    }
}
