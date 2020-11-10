use crate::errors::TokenizerError;
use std::convert::TryFrom;
use std::fmt::Display;

#[derive(Display, Debug, PartialEq, Eq, Clone)]
pub enum Token {
    /// This token should never be found in the token stream, it is used as an impossible value token
    #[allow(dead_code)]
    Infallible,
    /// An end of line ';'
    EOL,
    /// {
    LCurly,
    /// }
    RCurly,
    /// (
    LBracket,
    /// )
    RBracket,
    /// =
    Equals,
    /// >
    GreaterThan,
    /// <
    LessThan,
    /// -
    Minus,
    /// +
    Plus,
    /// *
    Times,
    /// / 
    Divide,
    /// !
    Exclamation,
    /// "
    DoubleQuote,
    /// A symbol, this should store the name of the symbol, and should match up to the symbol table
    Symbol(String),
    /// A constant, this should store the index of the constant in the constant table
    Constant(u32),
}

impl TryFrom<char> for Token {
    type Error = TokenizerError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let t = match c {
            ';' => Token::EOL,
            '{' => Token::LCurly,
            '}' => Token::RCurly,
            '(' => Token::LBracket,
            ')' => Token::RBracket,
            '"' => Token::DoubleQuote,
            '=' => Token::Equals,
            '>' => Token::GreaterThan,
            '<' => Token::LessThan,
            '-' => Token::Minus,
            '+' => Token::Plus,
            '*' => Token::Times,
            '/' => Token::Divide,
            '!' => Token::Exclamation,
            _ => Token::Infallible,
        };

        if t == Token::Infallible {
            Err(TokenizerError::new("Invalid token"))
        } else {
            Ok(t)
        }
    }
}
