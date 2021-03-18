#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("syntax rule for matched text not at start of string: {0:?}")]
    InvalidRule(crate::token::TokenType),
    #[error("invalid syntax somewhere in the source file")] // FIXME Improve error reporting
    Syntax,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Parse(#[from] ParseError),
}
