use self::prelude::*;

mod call;
mod import;
mod using;
#[allow(clippy::module_inception)]
mod instructions {
    pub use super::call::Call;
    pub use super::import::Import;
    pub use super::using::Using;
}

pub(self) mod prelude {
    pub use super::Instruction;
    pub use crate::expression::Expression;
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
