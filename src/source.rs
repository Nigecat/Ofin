#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// A possible source input for an ofin program.
pub trait Source: Sized {
    fn source(self) -> Result<String, Error>;
}

impl Source for String {
    fn source(self) -> Result<String, Error> {
        Ok(self)
    }
}

impl Source for std::path::PathBuf {
    fn source(self) -> Result<String, Error> {
        Ok(std::fs::read_to_string(self)?)
    }
}