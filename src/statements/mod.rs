mod assign;
pub use assign::Assign;

mod prelude {
    pub use super::Statement;
    pub use crate::error::Error;
    pub use crate::token::Token;
    pub use crate::token::TokenType::{self, *};
    pub use crate::vm::{Runnable, Value};
    pub use crate::Program;
    pub use std::fmt;
}
use self::prelude::*;
use crate::token::TokenStream;

pub trait Statement<const LENGTH: usize>: fmt::Debug {
    const TARGET: &'static [[TokenType; LENGTH]];

    /// Parse a set of tokens into this statement.
    ///
    /// The types of these tokens are guaranteed to equal one of the Self::TARGET elements
    fn parse(tokens: [Token; LENGTH]) -> Self;
}

// macro_rules! statements {
//     ($(statement: $ident),* $(,)?) => {
//         $(
            
//         )*
//     };
// }

// pub fn parse(tokens: TokenStream) -> Vec<Box<dyn Runnable>> {
    
// }
