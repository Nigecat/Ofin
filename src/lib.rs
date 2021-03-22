#![allow(incomplete_features)]
#![feature(const_generics)]

mod error;
mod expression;
mod source;
mod token;
mod vm;

pub use error::Error;
pub use ofin_std as std;
pub use source::Source;

use ::std::convert::TryInto;
use token::{TokenStream, TokenType};
use vm::parse_instructions;

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate lazy_static;

/// Run an ofin program from the supplied source.
#[tracing::instrument]
pub fn run(source: Source) -> Result<(), Error> {
    let contents: String = source.try_into()?;

    let mut tokens = TokenStream::lex(contents)?;
    tokens.filter(&[TokenType::Comment, TokenType::Control, TokenType::Space]);
    debug!("Tokens: \n{:#?}", tokens);

    let instructions = parse_instructions(tokens.tokens())?;
    debug!("Instructions: \n{:#?}", instructions);

    vm::run(instructions)?;
    Ok(())
}
