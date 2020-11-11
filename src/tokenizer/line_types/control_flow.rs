use super::super::TokenStreamToken;
use super::{LineResult, LineResultType};
use crate::tokens::Token;
use crate::errors::TokenizerError;
use std::convert::TryFrom;

/// Handle control flow lines in the form of `(if|while) Token::Expression`
pub fn control_flow(line: &[TokenStreamToken]) -> LineResultType {
    if line[0] == 'i' && line[1] == 'f' {
        let exp_pos = line.iter().position(|t| match t {
            TokenStreamToken::Token(Token::Expression(_)) => true,
            _ => false,
        }).ok_or_else(|| TokenizerError::new("If statement missing expression"))?;
        
        let exp = Token::try_from(&line[exp_pos]).unwrap();
        let ex = match exp {
            Token::Expression(e) => Some(e),
            _ => None,
        }.unwrap();

        return Ok(LineResult::new(true, Some([Token::If(ex)].to_vec())));
    } else if line[0] == 'w' && line[1] == 'h' && line[2] == 'i' && line[3] == 'l' && line[4] == 'e' {
        let exp_pos = line.iter().position(|t| match t {
            TokenStreamToken::Token(Token::Expression(_)) => true,
            _ => false,
        }).ok_or_else(|| TokenizerError::new("While loop missing expression"))?;
        
        let exp = Token::try_from(&line[exp_pos]).unwrap();
        let ex = match exp {
            Token::Expression(e) => Some(e),
            _ => None,
        }.unwrap();

        return Ok(LineResult::new(true, Some([Token::While(ex)].to_vec())));
    }

    Ok(LineResult::new(false, None))
}
