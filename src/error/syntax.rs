use std::fmt;

#[derive(Debug)]
pub struct SyntaxError();

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid syntax somewhere in the source file")
    }
}

impl std::error::Error for SyntaxError {}
