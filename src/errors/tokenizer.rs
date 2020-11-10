use std::fmt;

#[derive(Debug)]
pub struct TokenizerError {
    message: String,
}

impl TokenizerError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        TokenizerError {
            message: message.into(),
        }
    }
}

impl std::error::Error for TokenizerError {}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TokenizerError: {}", self.message)
    }
}
