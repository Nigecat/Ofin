mod error;
mod expression;
mod source;

pub use error::Error;
pub use ofin_std as std;
pub use source::Source;

use ::std::convert::TryInto;
use expression::Expression;

#[macro_use]
extern crate trace;
#[macro_use]
extern crate tracing;
// #[macro_use]
// extern crate lazy_static;

/// Run an ofin program from the supplied source.
pub fn run(source: Source) -> Result<(), Error> {
    let _contents: String = source.try_into()?;

    // let n = std::Integer::from(3);
    // println!("{:?}", n);

    // let input = "(-1+2)*3!";
    // let input = "-(1+2)*6/2";
    // // let input = "-(1+2)*7";
    // let expr = Expression::parse(input);
    // let result = expr.clone().evaluate();
    // print!("{} = {} = {}", input, expr, result);

    Ok(())
}
