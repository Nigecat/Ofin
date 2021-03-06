#[derive(Debug, thiserror::Error)]
pub enum OfinError {}

impl OfinError {
    /// Report this error
    pub fn report(&self) {
        error!("{:?}", self);
    }
}
