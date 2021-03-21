mod directive;
mod error;
mod source;
mod token;
mod vm;

pub use error::Error;
pub use ofin_std as std;
pub use source::Source;

use ::std::convert::TryInto;
use token::TokenStream;

#[macro_use]
extern crate tracing;

/// Run an ofin program from the supplied source.
#[tracing::instrument]
pub fn run(source: Source) -> Result<(), Error> {
    debug!("");
    let contents: String = source.try_into()?;
    let tokens = TokenStream::lex(contents);
    Ok(())
}
