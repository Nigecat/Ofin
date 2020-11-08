use crate::application::Application;
use crate::tokens::Token;

/// Tokenize an input string
pub fn tokenize(input: &str, application: &Application) -> Result<Vec<Token>, crate::error::Error> {
    let mut tokenstream = Vec::new();
    let chars = input.chars();

    // This will make the character enumerator skip the next n characters
    let mut skip: usize = 0;

    for (i, c) in chars.enumerate() {
        if skip > 0 {
            skip -= 1;
            trace!("Skipping character {} of input string...", i);
            continue
        }

        trace!("Processing character {} of input string: {}", i, c);

        // Convert this character if it is a valid single character token
        let token = single_char_token(c);
        if let Some(token) = token {
            trace!("Got token: {}", token);
            tokenstream.push(token);
        }





        // If we get this far then we must have gotten an invalid token
        tokenstream.push(Token::Infallible);
    }

    // If we have an illegal token in the tokenstream then throw an error
    if tokenstream.contains(&Token::Infallible) {
        Err(error!("Unknown token in source"))
    } else {
        Ok(tokenstream)
    }
}

/// Attempt to convert a single character into a token
fn single_char_token(c: char) -> Option<Token> {
    let token = match c {
        ';' => Token::EOL,
        _ => Token::Infallible,
    };

    if token == Token::Infallible {
        None
    } else {
        Some(token)
    }
}