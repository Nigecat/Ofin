use super::TokenStream;
use crate::application::Application;
use crate::errors::TokenizerError;

/// Tokenize expressions
pub fn pass_two(
    tokenstream: TokenStream,
    mut application: &mut Application,
) -> Result<TokenStream, TokenizerError> {
    debug!("Running pass 2 of tokenizer");

    Ok(tokenstream)
}
