mod stream;
mod t;
use crate::error::ParseError;
pub use stream::TokenStream;
pub use t::TokenType;

pub struct Token {
    t: TokenType,
    literal: String,
}

impl Token {
    fn new(t: TokenType, literal: String) -> Self {
        Token { t, literal }
    }
}
