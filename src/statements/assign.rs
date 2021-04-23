use super::prelude::*;

#[derive(Debug)]
pub struct Assign {
    var: rust::String,
    value: Value,
}

impl Statement for Assign {
    const TARGET: &'static [&'static [TokenType]] =
        &[&[Ident, Equals, String], &[Ident, Equals, Integer]];

    fn parse(mut tokens: Vec<Token>) -> Self {
        let mut value = tokens.remove(2);
        let var = tokens.remove(0).s;

        if value == String {
            Assign {
                var,
                value: Value::String(value.s),
            }
        } else if value == Integer {
            let value = if value.s.starts_with('-') {
                value.s.remove(0);
                Value::NegInteger(value.s.parse::<isize>().unwrap() * -1)
            } else {
                Value::PosInteger(value.s.parse().unwrap())
            };

            Assign { var, value }
        } else {
            unreachable!();
        }
    }
}

impl Runnable for Assign {
    fn run(&self, program: &mut State) -> Result<(), Error> {
        program
            .variables
            .insert(self.var.clone(), self.value.clone());
        Ok(())
    }
}
