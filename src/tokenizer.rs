use crate::tokens::Token;
use crate::application::Application;

/// Tokenize an input string
pub fn tokenize(input: &str, application: &Application) -> Vec<Token> {
    let tokenstream = Vec::new();

    for (i, c) in input.chars().enumerate() {
        println!("{} | {}", c, i);
    }


    tokenstream
}