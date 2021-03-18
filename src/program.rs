use crate::token::TokenStream;
use crate::Error;

pub struct Program {}

impl Program {
    pub fn parse(source: String) -> Result<Self, Error> {
        let _tokens = TokenStream::parse(source)?;

        Ok(Program {})
    }
}
