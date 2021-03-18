mod stream;
mod t;
use crate::error::ParseError;
use std::fmt;
pub use stream::TokenStream;
pub use t::TokenType;

pub struct Token {
    pub t: TokenType,
    pub literal: String, // TODO: Add unicode support
}

impl Token {
    fn new(t: TokenType, literal: String) -> Self {
        Token { t, literal }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:?})", self.t, self.literal)
    }
}
