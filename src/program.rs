use crate::token::Token;
use crate::Error;

pub struct Program {}

impl Program {
    pub fn parse(source: String) -> Result<Self, Error> {
        let _tokens = Token::parse(source)?;

        Ok(Program {})
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Error> {
        let source = std::fs::read_to_string(&path)?;
        let _tokens = Token::parse(source)?;

        Ok(Program {})
    }
}
