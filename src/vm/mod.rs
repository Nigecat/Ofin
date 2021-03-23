mod function;
mod instructions;
mod state;
mod value;
use crate::error::RuntimeError;
pub use instructions::parse_instructions;
pub use state::State;

pub type RunResult<T> = Result<T, RuntimeError>;

/// Something that can be run in the vm
pub trait Runnable: std::fmt::Debug {
    fn run(&self, state: &mut State) -> RunResult<()>;
}

pub fn run(instructions: Vec<Box<dyn Runnable>>) -> RunResult<()> {
    Ok(())
}
