use crate::{Error, Source};
use std::convert::TryInto;

/// A high level representation of an ofin program
pub struct Program {
    source: Source,
}

impl Program {
    fn new(source: Source) -> Self {
        Program { source }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        unimplemented!();
    }
}

impl<S: Into<Source>> From<S> for Program {
    fn from(source: S) -> Self {
        Program::new(source.into())
    }
}
