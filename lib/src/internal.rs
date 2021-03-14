#[derive(Debug, thiserror::Error)]
pub enum TransmuteError {

}

pub trait Transmute<T> {
    /// Convert `self` to `T`
    fn transmute(&self) -> Result<T, TransmuteError>;
}