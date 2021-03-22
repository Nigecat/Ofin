use super::prelude::*;

#[derive(Debug)]
pub struct Using(String);

impl Instruction<2> for Using {
    const FINGERPRINT: [TokenType; 2] = [TokenType::Using, TokenType::Target];

    fn parse(tokens: [Token; 2]) -> Self {
        let [_, target] = tokens;
        Using(target.literal())
    }
}
