use crate::application::Application;
use crate::tokens::Token;

#[derive(Eq, PartialEq)]
enum TokenStreamToken {
    Token(Token),
    Char(char),
}

/// Fully tokenize an input string
pub fn tokenize(input: &str, application: &mut Application) -> Result<Vec<Token>, crate::error::Error> {
    let tokenstream: Vec<TokenStreamToken> = Vec::new();


    // If there are any characters left after processing, then we must have invalid syntax somewhere
    for t in tokenstream {
        if let TokenStreamToken::Char == t {
            Err(error!("Syntax error"))
        }
    }

    Ok(tokenstream)
}
