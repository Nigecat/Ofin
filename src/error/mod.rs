mod syntax;
pub use syntax::SyntaxError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("invalid syntax somewhere in the source file")] // FIXME Improve error reporting
    Syntax(#[from] SyntaxError),
}
