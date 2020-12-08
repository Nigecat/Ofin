use thiserror::Error;

#[derive(Debug, Error, Clone, Copy)]
pub enum OfinError {
    #[error("rustc not found, please install it before attempting to run a script")]
    RustCNotFound,
}
