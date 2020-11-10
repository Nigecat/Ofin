mod pass_one;
mod tokenstreamtoken;
use crate::application::Application;
use crate::errors::TokenizerError;
use crate::tokens::Token;
use pass_one::pass_one;
use std::convert::TryFrom;
use tokenstreamtoken::TokenStreamToken;

type TokenStream = Vec<TokenStreamToken>;
type TokenStreamReturn = Result<TokenStream, TokenizerError>;

/// Fully tokenize an input string
pub fn tokenize(input: &str, application: &mut Application) -> Result<Vec<Token>, TokenizerError> {
    let mut tokenstream: Vec<TokenStreamToken>;

    // Pass 1: Process single character tokens (and ==)
    tokenstream = pass_one(&input)?;
    print!("{:#?}", tokenstream);

    // Pass 3: Allocate variables in the symbol table
    // Pass 4: Tokenize everything else

    Ok(vec![])
    /*
    // If there are any characters left after processing, then we must have invalid syntax somewhere
    if tokenstream.iter().any(|t| t.is_char()) {
        return Err(TokenizerError::new("Syntax error"));
    }

    // There should be no character in the token stream now, so we should be able to safetly unwrap each value into a token
    Ok(tokenstream
        .iter()
        .map(|t| Token::try_from(t).unwrap())
        .collect())
    */
}
