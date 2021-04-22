use super::prelude::*;

#[derive(Debug)]
pub struct Assign {}

impl Statement<3> for Assign {
    const TARGET: &'static [[TokenType; 3]] = &[
        [TokenType::Ident, TokenType::Equals, TokenType::String],
        [TokenType::Ident, TokenType::Equals, TokenType::Integer],
    ];

    fn parse(tokens: [Token; 3]) -> Self {
        unimplemented!();
    }
}
