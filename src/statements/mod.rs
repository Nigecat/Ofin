mod assign;
pub use assign::Assign;

mod prelude {
    pub use super::Statement;
    pub use crate::token::TokenType::*;
    pub use crate::token::{Token, TokenType};
    pub use std::fmt;
}
use prelude::*;

pub trait Statement<const LENGTH: usize>: fmt::Debug {
    const TARGET: &'static [[TokenType; LENGTH]];

    /// Parse a set of tokens into this statement.
    ///
    /// The types of these tokens are guaranteed to equal one of the Self::TARGET elements
    fn parse(tokens: [Token; LENGTH]) -> Self
    where
        Self: Sized;
}
