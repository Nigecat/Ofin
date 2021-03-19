use crate::error::SyntaxError;
use crate::token::{Token, TokenType};

#[deny(dead_code)]
#[derive(Debug, Clone)]
/// A higher level representation of ofin source code, this is completely decoupled from the syntax
pub enum Directive {
    Import(String),
    Using(String),
}

macro_rules! register {
    ($tokens: expr, $directives: expr, $directive: expr => $index: literal, [$($t: expr),*]) => {
        let tokens = &[$($t),*];
        if crate::utils::first(&$tokens, tokens) {
            let elements = $tokens.drain(0..tokens.len());
            let literal = elements.into_iter().nth($index).unwrap().literal;
            $directives.push($directive(literal));
            continue;
        }
    };
}

impl Directive {
    pub fn parse(mut t: Vec<Token>) -> Result<Vec<Self>, SyntaxError> {
        let mut d = Vec::new();

        while !t.is_empty() {
            register!(t, d, Directive::Import => 1, [TokenType::Import, TokenType::Target]);
            register!(t, d, Directive::Using => 1, [TokenType::Using, TokenType::Target]);

            return Err(SyntaxError());
        }

        debug!("Parsed directives: {:#?}", d);

        Ok(d)
    }
}
