use crate::error::ParseError;
use crate::language::syntax::{SYNTAX, SYNTAX_CLEAN};
use std::fmt;

#[deny(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    Ident,
    Number,

    // ----- Keywords -----
    Import,
    Using,

    // ----- Characters -----
    Space,
    LBracket,
    RBracket,

    // ----- Special Characters -----
    Eol,
    Control,

    // ----- Blocks -----
    Target,
    Comment,
}

impl TokenType {
    /// Identify the token contained at the start of the string along with it's length
    pub(super) fn identify(s: &str) -> Option<(Self, usize)> {
        for rule in SYNTAX.iter() {
            let (rule, t) = rule;
            if let Some((start, end)) = rule.find(s) {
                // The rule should only match the start of the string
                assert_eq!(start, 0, "{}", ParseError::InvalidRule(*t));

                return Some((*t, end));
            }
        }

        None
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Token {
    pub t: TokenType,
    pub literal: String, // TODO: Add unicode support
}

impl Token {
    fn new(t: TokenType, literal: String) -> Self {
        Token { t, literal }
    }

    /// Convert a string into a vector of tokens
    pub fn parse(mut source: String) -> Result<Vec<Token>, ParseError> {
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

        // Remove the useless tokens
        tokens.retain(|token| !SYNTAX_CLEAN.contains(&token.t));

        debug!("Parsed tokens: {:#?}", tokens);

        Ok(tokens)
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:?})", self.t, self.literal)
    }
}
