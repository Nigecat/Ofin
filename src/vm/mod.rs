mod instructions;
use crate::error::RuntimeError;
pub use instructions::parse_instructions;

/// Something that can be run in the vm
pub trait Runnable: std::fmt::Debug {}

pub fn run(instructions: Vec<Box<dyn Runnable>>) -> Result<(), RuntimeError> {
    Ok(())
}
