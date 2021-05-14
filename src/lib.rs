mod error;
mod source;
mod token;

pub use error::Error;
pub use source::Source;

use crate::token::TokenType;
use token::lex;

pub fn run<S: Into<Source>>(source: S) -> Result<(), Error> {
    let source = source.into();

    let mut tokens = lex(source)?;
    // Remove tokens that don't have any syntactical meaning
    tokens.retain(|token| {
        !vec![
            TokenType::Space,
            TokenType::Newline,
            TokenType::Tab,
            TokenType::Eof, // FIXME: Remove this line when allowing multi-file imports
        ]
        .contains(&token.t)
    });
    println!("{:#?}", tokens);

    Ok(())
}
