mod error;
mod source;
mod token;

pub use error::Error;
pub use source::Source;

use token::lex;

pub fn run<S: Into<Source>>(source: S) -> Result<(), Error> {
    let source = source.into();

    let mut tokens = lex(source)?;
    // Remove tokens that don't have any syntactical meaning
    tokens.retain(|token| !token::MEANINGLESS.contains(&token.t));
    println!("{}", token::rend(&tokens));

    Ok(())
}
