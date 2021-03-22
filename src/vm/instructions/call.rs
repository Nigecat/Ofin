use super::prelude::*;
use crate::expression::Expression;

/// A function call
#[derive(Debug)]
pub struct Call {
    function: String,
    args: Expression,
}

impl Instruction<2> for Call {
    const FINGERPRINT: [TokenType; 2] = [TokenType::Ident, TokenType::Expression];

    fn parse(tokens: [Token; 2]) -> Self {
        let [function, args] = tokens;
        let function = function.literal();
        let args = args.expression().unwrap();
        Call { function, args }
    }
}

impl Runnable for Call {}
