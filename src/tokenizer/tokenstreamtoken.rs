use crate::tokens::Token;
use std::convert::TryFrom;

#[derive(Eq, PartialEq)]
pub enum TokenStreamToken {
    Token(Token),
    Char(char),
}

impl TokenStreamToken {
    pub fn is_char(&self) -> bool {
        match self {
            TokenStreamToken::Char(_) => true,
            _ => false,
        }
    }
}

impl TryFrom<&TokenStreamToken> for Token {
    type Error = crate::error::Error;

    fn try_from(t: &TokenStreamToken) -> Result<Self, Self::Error> {
        match t {
            TokenStreamToken::Char(_) => Err(error!("Token parsing error")),
            TokenStreamToken::Token(token) => Ok(token.clone()),
        }
    }
}
