use super::super::TokenStreamToken;
use super::{LineResult, LineResultType};
use crate::tokens::Token;

/// Handle singular curly brackets: `{` or `}`
pub fn curly_bracket(line: &[TokenStreamToken]) -> LineResultType {
    if &line[0] == Token::LCurly {
        Ok(LineResult::new(true, Some([Token::LCurly].to_vec())))
    } else if &line[0] == Token::RCurly {
        Ok(LineResult::new(true, Some([Token::RCurly].to_vec())))
    } else {
        Ok(LineResult::new(false, None))
    }
}
