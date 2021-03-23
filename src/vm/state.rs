use super::Runnable;
use crate::error::RuntimeError;

/// The state of an ofin program
#[derive(Default)]
pub struct State {}

impl State {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn exec(&mut self, instruction: Box<dyn Runnable>) -> Result<(), RuntimeError> {
        Ok(())
    }
}
