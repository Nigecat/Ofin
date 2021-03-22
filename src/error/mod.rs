mod runtime;
mod syntax;
pub use runtime::RuntimeError;
pub use syntax::SyntaxError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Syntax(#[from] SyntaxError),
    #[error(transparent)]
    Runtime(#[from] RuntimeError),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
