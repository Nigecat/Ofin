#![deny(dead_code)]

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
use std::convert::TryInto;

pub trait Instruction<const LENGTH: usize>: Runnable {
    const FINGERPRINT: [TokenType; LENGTH];

    fn check(tokens: &[Token; LENGTH]) -> bool {
        tokens == &Self::FINGERPRINT
    }

    fn parse(tokens: [Token; LENGTH]) -> Self;
}

macro_rules! register {
    ($potential: ident, $length: literal, [$( $instruction: ident ),*]) => {
        {
            let tokens: [Token; $length] = $potential.try_into().unwrap();
            // Null if statement to allow us to use `else if` in the repititions
            if false {
                None
            }
            $(
                else if <$instruction>::check(&tokens) {
                    Some(Some(Box::new(<$instruction>::parse(tokens))))
                }
            )*
            else {
                None
            }
        }
    };
}

pub fn parse_instructions(tokens: Vec<Token>) -> Result<Vec<Box<dyn Runnable>>, SyntaxError> {
    // Chunk the tokens on each TokenType::Eol character
    let mut instructions_source: Vec<Vec<Token>> = vec![Vec::new()];

    for token in tokens.into_iter() {
        if token == TokenType::Eol {
            instructions_source.push(Vec::new());
        } else {
            instructions_source.last_mut().unwrap().push(token);
        }
    }

    // The last element should be an empty array since the program should end in a `;`
    if !instructions_source.last().unwrap().is_empty() {
        return Err(SyntaxError::new());
    } else {
        // Remove the empty element
        instructions_source.remove(instructions_source.len() - 1);
    }

    debug!("Split tokens into: \n{:#?}", instructions_source);

    let mut instructions = Vec::new();
    for potential in instructions_source.into_iter() {
        let instruction: Option<Option<Box<dyn Runnable>>>;
        instruction = match potential.len() {
            0 => None, // Ignore zero length instructions, this is from two semicolons in a row (;;) and is effectively a nop
            2 => register!(potential, 2, [Call, Using, Import]),
            _ => panic!("unimplemented instruction length"),
        };

        if let Some(instruction) = instruction {
            if let Some(instruction) = instruction {
                instructions.push(instruction);
            } else {
                return Err(SyntaxError::new());
            }
        }
    }

    Ok(instructions)
}
