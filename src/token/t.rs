use crate::error::ParseError;
use crate::language::SYNTAX;
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
            if let Some(re) = rule.find(s) {
                // The rule should only match the start of the string
                assert_eq!(re.start(), 0, "{}", ParseError::InvalidRule(*t));

                return Some((*t, re.end()));
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
