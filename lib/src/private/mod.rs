mod error;
pub mod transmute;
pub use error::Error;
pub use transmute::Transmutable;

/// # Examples
///
/// ```rust
/// call("sleep", &["time"], &[Box::new(1000)]);
/// ```
pub fn call(
    _name: &str,
    _namespace: &[&str],
    _args: &[Transmutable],
) -> Result<Option<Transmutable>, Error> {
    unimplemented!();
}
