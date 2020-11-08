use crate::application::Application;
use crate::tokens::Token;

/// Tokenize an input string
pub fn tokenize(input: &str, application: &Application) -> Vec<Token> {
    let tokenstream = Vec::new();

    for (i, c) in input.chars().enumerate() {
        trace!("Processing character {} of input string: {}", i, c);
    }

    tokenstream
}
