use super::{ParseError, Token, TokenType};
use std::convert::TryFrom;
use std::str::FromStr;

pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn parse(mut source: String) -> Result<Self, ParseError> {
        let mut tokens = Vec::new();

        // Iterate over the tokens until we run out
        while let Some((t, length)) = TokenType::identify(&source) {
            let literal = source.drain(0..length);
            tokens.push(Token::new(t, literal.collect()));
        }

        // If we have any characters left then there is a syntax error
        if !source.is_empty() {
            return Err(ParseError::Syntax);
        }

        Ok(TokenStream { tokens })
    }
}

impl TryFrom<String> for TokenStream {
    type Error = ParseError;

    fn try_from(source: String) -> Result<Self, Self::Error> {
        TokenStream::parse(source)
    }
}

impl FromStr for TokenStream {
    type Err = ParseError;

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        TokenStream::parse(source.to_string())
    }
}
