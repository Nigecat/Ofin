use super::Runnable;
use crate::error::RuntimeError;

/// The state of an ofin program
#[derive(Default)]
pub struct State {}

impl State {
    pub fn new() -> Self {
        Default::default()
    }

    /// Run an instruction
    pub fn exec(&mut self, instruction: Box<dyn Runnable>) -> Result<(), RuntimeError> {
        instruction.run(self)?;
        Ok(())
    }
}
