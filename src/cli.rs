use crate::errors::CliError;
use std::fs;

pub struct CLI {
    pub path: String,
}

impl CLI {
    pub fn parse() -> Result<Self, CliError> {
        let path = match std::env::args().nth(1) {
            Some(p) => Ok(p),
            None => Err(CliError::new("no input file")),
        };

        let path = path?;

        // Make sure the source file exists
        if fs::metadata(&path).is_err() {
            return Err(CliError::new(format!(
                "{}: no such file or directory",
                path
            )));
        }

        Ok(CLI { path })
    }
}
