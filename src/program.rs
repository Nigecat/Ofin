use crate::Error;
use std::str::FromStr;

pub struct Program {}

impl Program {
    pub fn parse(source: &str) -> Result<Self, Error> {
        Ok(Program {})
    }
}

impl FromStr for Program {
    type Err = Error;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Program::parse(source)
    }
}
