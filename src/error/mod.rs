#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("file not found: {0}")]
    FileNotFound(std::path::PathBuf),
    #[error("an unexpected io error occured: {0}")]
    IoError(#[from] std::io::Error),
    #[error("unexpectd syntax: {0}")] // TOOD: Improve this error message
    SynaxError(String),
}

impl Error {
    /// Report this error
    pub fn report(&self) {
        if tracing::dispatcher::has_been_set() {
            error!("{}", self);
        } else {
            println!("error: {}", self);
        }
    }
}
