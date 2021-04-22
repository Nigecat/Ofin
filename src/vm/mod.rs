mod value;

pub use value::Value;

pub trait Runnable {
    fn run(&self, program: &mut crate::Program) -> Result<(), crate::error::Error>;
}
