mod syntax;
pub use syntax::SyntaxError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Syntax(#[from] SyntaxError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
