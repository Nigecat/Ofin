use super::line_types::*;
use super::{TokenStream, TokenStreamReturn, TokenStreamToken};
use crate::application::Application;
use crate::errors::TokenizerError;
use crate::tokens::Token;

/// Allocate variables/constants in the symbol/constant table
pub fn pass_two(tokenstream: TokenStream, mut application: &mut Application) -> TokenStreamReturn {
    debug!("Running pass 2 of tokenizer");

    let mut lines: Vec<TokenStream> = Vec::new();
    // Add an entry for the first line
    lines.push(Vec::new());

    // Iterate over the tokens and add a new element to the array every newline
    for t in tokenstream {
        if t == TokenStreamToken::Token(Token::EOL) {
            lines.push(Vec::new());
        } else {
            // Take out any leading spaces
            // (trailing spaces are impossible since each line ends on the semicolon,
            //         and the entire source string was trimmed before getting passed to the tokenizer)
            if !(lines.last().unwrap().is_empty() && t == TokenStreamToken::Char(' ')) {
                lines.last_mut().unwrap().push(t);
            }
        }
    }
    // Assuming the user put a semicolon on the last line the last element should be empty
    if !lines.last().unwrap().is_empty() {
        return Err(TokenizerError::new("Missing semicolon on last line"));
    }

    // Now we can safetly remove the last line without losing any data
    lines.remove(lines.len() - 1);

    debug!(" - Detected {} lines", lines.len());

    // Now we can process things line by line
    for (i, line) in lines.iter().enumerate() {
        trace!("Processing line {}", i + 1);
        super::print_tokenstream(&line);

        if declare_variable(&line, &mut application)? {
            continue;
        }
        // If we couldn't match this line then we know that it is a syntax error
        else {
            return Err(TokenizerError::new(format!("Invalid syntax on line {} (this count excludes blank lines)", i)));
        }
    }

    Ok(vec![])
}
