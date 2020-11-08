use crate::application::Application;
use crate::tokens::Token;

/// Tokenize an input string
pub fn tokenize(input: &str, application: &Application) -> Vec<Token> {
    let tokenstream = Vec::new();

    for (i, c) in input.chars().enumerate() {
        println!("{} | {}", c, i);
    }

    tokenstream
}
