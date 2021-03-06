use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    LParen,
    RParen,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Minu,
    Plus,
    Semicolon,
    Slash,
    Star,

    Exclamation,
    ExclamationEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    If,
    Else,
    And,
    False,
    True,
    For,
    Or,
    Return,
    Let,
    Func,

    Eof,
}

pub struct Token {
    pub t: TokenType,
    pub row: usize,
    pub column: usize,
}

impl Token {
    pub fn new(t: TokenType, row: usize, column: usize) -> Self {
        Token { t, row, column }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} at line {}:{}", self.t, self.row, self.column)
    }
}
