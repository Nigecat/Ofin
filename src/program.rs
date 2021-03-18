use crate::directive::Directive;
use crate::token::Token;
use crate::Error;

pub struct Program {}

impl Program {
    pub fn parse(source: String) -> Result<Self, Error> {
        let tokens = Token::parse(source)?;
        let _directives = Directive::parse(tokens)?;

        Ok(Program {})
    }

    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Error> {
        let source = std::fs::read_to_string(&path)?;
        let tokens = Token::parse(source)?;
        let _directives = Directive::parse(tokens)?;

        Ok(Program {})
    }
}
