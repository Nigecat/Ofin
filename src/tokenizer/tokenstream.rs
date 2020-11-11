use crate::errors::TokenizerError;
use crate::tokens::Token;
use std::convert::TryFrom;

pub type TokenStream = Vec<TokenStreamToken>;
pub type TokenStreamReturn = Result<TokenStream, TokenizerError>;

#[derive(Debug, Eq, PartialEq, Clone)]
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

impl PartialEq<Token> for &TokenStreamToken {
    fn eq(&self, other: &Token) -> bool {
        match self {
            TokenStreamToken::Char(_) => false,
            TokenStreamToken::Token(t) => t == other,
        }
    }
}

impl PartialEq<char> for TokenStreamToken {
    fn eq(&self, other: &char) -> bool {
        self == &TokenStreamToken::from(other)
    }
}

impl From<Token> for TokenStreamToken {
    fn from(t: Token) -> Self {
        TokenStreamToken::Token(t)
    }
}

impl From<&char> for TokenStreamToken {
    fn from(c: &char) -> Self {
        TokenStreamToken::Char(*c)
    }
}

impl TryFrom<&TokenStreamToken> for char {
    type Error = TokenizerError;

    fn try_from(t: &TokenStreamToken) -> Result<Self, Self::Error> {
        match t {
            TokenStreamToken::Char(c) => Ok(*c),
            TokenStreamToken::Token(_) => Err(TokenizerError::new("Token parsing error")),
        }
    }
}

impl TryFrom<&TokenStreamToken> for Token {
    type Error = TokenizerError;

    fn try_from(t: &TokenStreamToken) -> Result<Self, Self::Error> {
        match t {
            TokenStreamToken::Char(_) => Err(TokenizerError::new("Token parsing error")),
            TokenStreamToken::Token(token) => Ok(token.clone()),
        }
    }
}

pub fn tokenstream_to_string(tokenstream: &TokenStream) -> Result<String, TokenizerError> {
    let mut chars: Vec<char> = Vec::new();
    for t in tokenstream {
        chars.push(char::try_from(t)?);
    }

    Ok(chars.iter().cloned().collect())
}
