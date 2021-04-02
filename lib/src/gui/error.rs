/// A possible GUI error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An active window already exists
    #[error("An active window already exists, call `closeWindow()` to close it.")]
    WindowExists,
    /// No active window 
    #[error("This method requires a window to be active, call `createWindow()` to create one.")]
    WindowNotFound,
}
