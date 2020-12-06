use thiserror::Error;

#[derive(Debug, Error)]
pub enum OfinError {
    #[error("rustc not found, please install it before attempting to run a script")]
    RustCNotFound,
}
