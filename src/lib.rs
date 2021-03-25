mod error;
mod expression;
mod source;

pub use error::Error;
pub use ofin_std as std;
pub use source::Source;

use ::std::convert::TryInto;
use expression::Expression;

// #[macro_use]
// extern crate tracing;
// #[macro_use]
// extern crate lazy_static;

/// Run an ofin program from the supplied source.
#[tracing::instrument]
pub fn run(source: Source) -> Result<(), Error> {
    let _contents: String = source.try_into()?;

    let expr = Expression::parse("(-1+2)*3");
    println!("{}", expr);

    Ok(())
}
