mod rule;
use rule::Rule;

use crate::error::SyntaxError;
use std::ops::Deref;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TokenType {
    Target,
}

macro_rules! rule {
    ($t: expr, $re: literal) => {
        Rule::new($t, $re)
    };
}

lazy_static! {
    static ref RULES: Vec<Rule> = vec![rule!(TokenType::Target, "<.*?>"),];
}

pub struct Token {
    t: TokenType,
    literal: String,
}

impl Token {
    pub fn literal(&self) -> &str {
        &self.literal
    }
}

impl Deref for Token {
    type Target = TokenType;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn lex(source: String) -> Result<Self, SyntaxError> {
        unimplemented!();
    }
}
