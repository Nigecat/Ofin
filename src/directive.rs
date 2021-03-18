use crate::error::SyntaxError;
use crate::token::Token;

/// A higher level representation of ofin source code, this is completely decoupled from the syntax
pub enum Directive {
    Import(String),
    Using(String),
}

impl Directive {
    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Self>, SyntaxError> {
        Ok(Default::default())
    }
}
