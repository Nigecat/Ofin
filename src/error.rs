#[derive(Debug, thiserror::Error)]
pub enum OfinError {}

impl OfinError {
    /// Report this error
    pub fn report(&self) {
        if tracing::dispatcher::has_been_set() {
            error!("{:?}", self);
        } else {
            println!("error: {:#?}", self);
        }
    }
}
