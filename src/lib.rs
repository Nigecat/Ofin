pub mod build;
mod error;
pub use error::OfinError;

#[macro_use]
extern crate tracing;

/// Run a script
#[tracing::instrument]
pub fn run<S: AsRef<str> + std::fmt::Debug>(script: S) -> Result<(), error::OfinError> {
    info!("");

    let _script = script.as_ref();

    Ok(())
}
