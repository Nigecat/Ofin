use std::fmt;

#[derive(Debug)]
pub struct CliError {
    message: String,
}

impl CliError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        CliError {
            message: message.into(),
        }
    }
}

impl std::error::Error for CliError {}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CliError: {}", self.message)
    }
}
