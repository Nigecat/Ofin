mod matcher;
mod token_type;
use matcher::TokenMatcher;
use token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    /// The type of this token
    pub t: TokenType,
    /// The string literal of this token
    pub literal: String,
}

impl Token {
    /// Attempt to convert the start of a string into a token
    pub fn from_str_start<S: AsRef<str>>(s: S) -> Option<Token> {
        let token = TokenMatcher::find(&s)?;
        let literal = &s.as_ref()[..token.l];

        Some(Token {
            t: token.t,
            literal: literal.to_string(),
        })
    }
}
