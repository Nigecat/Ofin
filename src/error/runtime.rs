#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
