use super::types::Type;
use super::value::Value;
use std::marker::PhantomData;
use crate::error::RuntimeError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Runtime(#[from] RuntimeError),
}

/// An ofin function
pub struct Function<R: Type> {
    /// The arguments to this function
    args: Vec<Box<dyn Type>>,
    phantom: PhantomData<R>,
}

impl<R: Type> Function<R> {
    pub fn new(args: Vec<Box<dyn Type>>, result: R) -> Self {
        Function {
            args,
            phantom: PhantomData,
        }
    }

    pub fn call(args: Vec<Box<dyn Type>>) -> Result<Value<R>, Error> {
        unimplemented!();
    }
}
