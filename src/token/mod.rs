mod matcher;
mod stream;
pub use matcher::TokenMatcher;
pub use stream::TokenStream;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {}

pub struct Token {
    t: TokenType,
    value: String,
}
