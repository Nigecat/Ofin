mod syntax;
pub use syntax::SyntaxError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] ::std::io::Error),
    #[error("{0}")]
    Syntax(#[from] SyntaxError),
}
