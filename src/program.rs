use crate::token::TokenStream;
use crate::Error;

pub struct Program {}

impl Program {
    pub fn parse(source: String) -> Result<Self, Error> {
        let tokens = TokenStream::parse(source)?;
        trace!("Parsed tokens: {:#?}", tokens);

        Ok(Program {})
    }
}
