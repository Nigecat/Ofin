use super::prelude::*;

#[derive(Debug)]
pub struct Call {
    f: Function,
    args: vm::Expression,
}

impl Statement for Call {
    const TARGET: Target = &[&[Ident, Expression]];

    fn parse(tokens: Vec<Token>) -> Result<Self, Error> {
        let name = tokens.remove(0).s;
        let args = tokens.remove(0).s;
        Call {
            f: Function::new(name),
            args: vm::Expression::parse(args)? ,// todo
        }
    }
}

impl Runnable for Call {
    fn run(&self, program: &mut State) -> Result<(), crate::error::Error> {
        Ok(())
    }
}
