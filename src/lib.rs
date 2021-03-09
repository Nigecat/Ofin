mod error;
mod lexer;
mod token;
pub use error::OfinError;
use token::Token;

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate lazy_static;

/// Run a script
pub fn run(script: String) -> Result<(), error::OfinError> {
    debug!("Running script:\n{}", script);

    let _tokens = lexer::lex(script)?;

    Ok(())
}
