use self::prelude::*;

pub mod prelude {
    pub use super::Instruction;
    pub use crate::token::{Token, TokenType};
    pub use std::fmt;
}

pub trait Instruction<const LENGTH: usize>: fmt::Debug
where
    Self: Sized,
{
    const FINGERPRINT: [TokenType; LENGTH];

    fn check(tokens: &[Token; LENGTH]) -> bool {
        tokens == &Self::FINGERPRINT
    }

    fn parse(tokens: [Token; LENGTH]) -> Self;
}
