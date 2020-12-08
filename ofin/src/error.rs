use thiserror::Error;

#[derive(Debug, Error, Clone, Copy)]
pub enum OfinError {
    #[error("syntax error")]
    SyntaxError,
}
