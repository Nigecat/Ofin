use super::{ParseError, Token, TokenType};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
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

        let stream = TokenStream { tokens };
        trace!("Parsed tokens: {:#?}", stream);

        Ok(stream)
    }
}

impl Deref for TokenStream {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.tokens
    }
}

impl DerefMut for TokenStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tokens
    }
}
