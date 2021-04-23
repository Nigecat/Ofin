mod assign;

use assign::Assign;

mod prelude {
    pub use super::Statement;
    pub use crate::error::Error;
    pub use crate::token::Token;
    pub use crate::token::TokenType::{self, *};
    pub use crate::vm::{Runnable, Value, State};
    pub use crate::Program;
    pub use std::fmt;
}
use self::prelude::*;
use crate::token::TokenStream;

pub trait Statement: fmt::Debug {
    const TARGET: &'static [&'static [TokenType]];

    /// Parse a set of tokens into this statement.
    ///
    /// The types of these tokens are guaranteed to equal one of the Self::TARGET elements
    fn parse(tokens: Vec<Token>) -> Self;
}

pub fn parse(mut tokens: TokenStream) -> Result<Vec<Box<dyn Runnable>>, usize> {
    assert_eq!(
        tokens.slice(tokens.len() - 1..tokens.len()),
        [TokenType::Semicolon],
        "Tokenstream missing ending semicolon, this should have been caught by the lexer."
    );

    let mut statements: Vec<Box<dyn Runnable>> = Vec::new();
    let mut original_length = tokens.len();

    'parser: while !tokens.is_empty() {
        let eol = tokens.position(TokenType::Semicolon).unwrap();
        tokens.remove(eol);
        let statement_tokens = tokens.slice(0..eol);

        macro_rules! register {
            ([ $($statement: ident),* $(,)? ]) => {
                $(
                    for target in $statement::TARGET {
                        if statement_tokens == *target {
                            original_length += target.len();
                            let t = tokens.rem(0..target.len());
                            let statement = $statement::parse(t);
                            statements.push(Box::new(statement));
                            continue 'parser;
                        }
                    }
                )*
            };
        }

        register!([Assign]);

        return Err(original_length - tokens.len());
    }

    Ok(statements)
}
