use std::fmt;

pub struct SyntaxError {
    line: usize,
    message: String,
    r#where: &'static str,
}

impl ::std::error::Error for SyntaxError {}

impl SyntaxError {
    pub fn new(line: usize, r#where: &'static str, message: String) -> Self {
        SyntaxError {
            line,
            message,
            r#where,
        }
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}] Error in {}: {}",
            self.line, self.r#where, self.message
        )
    }
}

impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
