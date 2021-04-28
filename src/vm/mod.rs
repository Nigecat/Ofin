mod function;
mod state;
mod value;
mod expression;

use crate::Error;
pub use function::Function;
pub use state::State;
pub use value::Value;
pub use expression::Expression;

pub trait Runnable: std::fmt::Debug {
    fn run(&self, program: &mut State) -> Result<(), crate::error::Error>;
}

pub fn run(statements: Vec<Box<dyn Runnable>>) -> Result<(), Error> {
    let mut state = Default::default();

    for statement in statements {
        statement.run(&mut state)?;
    }

    Ok(())
}
