mod call;
mod import;
mod using;
use call::Call;
use import::Import;
use using::Using;

mod prelude {
    pub use super::super::Runnable;
    pub use super::Instruction;
    pub use crate::token::{Token, TokenType};
}

use self::prelude::*;
use crate::error::SyntaxError;

pub trait Instruction<const LENGTH: usize>: Runnable {
    const FINGERPRINT: [TokenType; LENGTH];

    fn check(tokens: &[Token; LENGTH]) -> bool {
        tokens == &Self::FINGERPRINT
    }

    fn parse(tokens: [Token; LENGTH]) -> Self;
}

pub fn parse_instructions(mut tokens: Vec<Token>) -> Result<Vec<Box<dyn Runnable>>, SyntaxError> {
    Err(SyntaxError::new())
}
