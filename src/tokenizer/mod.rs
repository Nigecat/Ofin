mod line_types;
mod pass_one;
mod pass_two;
mod tokenstream;
use crate::application::Application;
use crate::errors::TokenizerError;
use crate::tokens::Token;
use pass_one::pass_one;
use pass_two::pass_two;
use std::convert::TryFrom;
use tokenstream::*;
use tokenstream::{TokenStream, TokenStreamToken};

/// Take an array of tokens and remove any leading or trailing whitespace
///
/// If tokenstream is an array filled with only spaces it will return an empty array.
/// NOTE: This modifies the original token stream
fn clean(tokenstream: TokenStream) -> TokenStream {
    let mut tokens = tokenstream.clone();
    // Remove elements from the start until we either run out of elements or get to a character that isn't a space
    while !tokens.is_empty() && tokens.first().unwrap() == &TokenStreamToken::Char(' ') {
        tokens.remove(0);
    }
    // Same as above but for the end
    while !tokens.is_empty() && tokens.last().unwrap() == &TokenStreamToken::Char(' ') {
        tokens.remove(tokens.len() - 1);
    }

    tokens
}

/// Fully tokenize an input string
pub fn tokenize(
    input: &str,
    mut application: &mut Application,
) -> Result<Vec<Token>, TokenizerError> {
    let mut tokenstream: Vec<TokenStreamToken>;

    // Pass 1: Process single character tokens (and ==)
    tokenstream = pass_one(&input)?;

    // Pass 2: Allocate variables/constants in the symbol/constant table
    tokenstream = pass_two(tokenstream, &mut application)?;

    println!("{:?}", tokenstream);
    println!("{:?}", application);
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
