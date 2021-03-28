mod binding;
mod evaluator;
mod lexer;

use lexer::ExpressionLexer;
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct Expression {
    s: lexer::S,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Expression {
    /// Parse the given input to an expression
    pub fn parse<S: AsRef<str>>(input: S) -> Self {
        ExpressionLexer::lex(input)
    }

    /// Evaluate this expression
    pub fn evaluate(self) -> f32 {
        evaluator::evaluate(self)
    }
}
