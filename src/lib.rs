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

    // let input = "(-1+2)*3!";
    let input = "(-1+2)*3";
    // let input = "(-1+2)*3";
    print!("{} = ", input);
    let expr = Expression::parse(input);
    print!("{} = ", expr);
    let result = expr.evaluate();
    print!("{}", result);

    Ok(())
}
