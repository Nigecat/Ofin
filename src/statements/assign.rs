use super::prelude::*;

#[derive(Debug)]
pub struct Assign {}

impl Statement for Assign {
    const TARGET: &'static [&'static [TokenType]] =
        &[&[Ident, Equals, String], &[Ident, Equals, Integer]];

    fn parse(tokens: Vec<Token>) -> Self {
        // unimplemented!();
        Assign {}
    }
}

impl Runnable for Assign {
    fn run(&self, program: &mut Program) -> Result<(), Error> {
        unimplemented!();
    }
}
