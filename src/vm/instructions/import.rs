use super::prelude::*;

#[derive(Debug)]
pub struct Import(String);

impl Instruction<2> for Import {
    const FINGERPRINT: [TokenType; 2] = [TokenType::Import, TokenType::Target];

    fn parse(tokens: [Token; 2]) -> Self {
        let [_, target] = tokens;
        Import(target.literal())
    }
}
