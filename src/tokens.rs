use crate::errors::TokenizerError;
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expression(Vec<Token>);

impl Expression {
    pub fn new(tokens: Vec<Token>) -> Self {
        Expression(tokens)
    }

    pub fn evaluate(&self) -> usize {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
    /// ,
    Comma,
    /// !
    Exclamation,
    /// "
    DoubleQuote,
    /// An expression
    Expression(Expression),
    /// An if statement and the expression that follows it
    If(Expression),
    /// A while loop and the expression that follows it
    //While(Expression),
    /// A symbol, this should store the name of the symbol, and should match up to the symbol table
    Symbol(String),
    /// A constant, this should store the index of the constant in the constant table
    Constant(usize),
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
            ',' => Token::Comma,
            _ => Token::Infallible,
        };

        if t == Token::Infallible {
            Err(TokenizerError::new("Unable to convert character to token"))
        } else {
            Ok(t)
        }
    }
}

impl TryFrom<&Token> for char {
    type Error = TokenizerError;

    fn try_from(t: &Token) -> Result<Self, Self::Error> {
        let c = match t {
            Token::EOL => ';',
            Token::LCurly => '{',
            Token::RCurly => '}',
            Token::LBracket => '(',
            Token::RBracket => ')',
            Token::DoubleQuote => '\"',
            Token::Equals => '=',
            Token::GreaterThan => '>',
            Token::LessThan => '<',
            Token::Minus => '-',
            Token::Plus => '+',
            Token::Times => '*',
            Token::Divide => '/',
            Token::Exclamation => '!',
            Token::Comma => ',',
            _ => ' ',
        };

        if c == ' ' {
            Err(TokenizerError::new("Unable to convert token to character"))
        } else {
            Ok(c)
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Ok(c) = char::try_from(self) {
            write!(f, "{}", c)
        } else {
            write!(f, "{:?}", self)
        }
    }
}
