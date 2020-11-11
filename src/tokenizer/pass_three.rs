use super::line_types::*;
use super::{tokenstream_display, TokenStream, TokenStreamToken};
use crate::application::Application;
use crate::errors::TokenizerError;
use crate::tokens::Token;

/// Tokenize everything else
pub fn pass_three(
    tokenstream: TokenStream,
    mut application: &mut Application,
) -> Result<Vec<Token>, TokenizerError> {
    debug!("Running pass 3 of tokenizer");

    let mut lines: Vec<TokenStream> = Vec::new();
    // Add an entry for the first line
    lines.push(Vec::new());

    // Iterate over the tokens and add a new element to the array every newline
    for t in tokenstream {
        // Put curly brackets in their own lines
        if &t == Token::LCurly || &t == Token::RCurly {
            lines.push(Vec::new());
            lines.last_mut().unwrap().push(t);
            lines.push(Vec::new());
        } else if t == TokenStreamToken::Token(Token::EOL) {
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

    let mut tokens: Vec<Token> = Vec::new();

    // Now we can process things line by line (and skip any empty lines)
    for (i, line) in lines.iter().filter(|l| !l.is_empty()).enumerate() {
        trace!("Processing line {}", i + 1);

        if declare_variable(&line, &mut application)?.status {
            continue;
        }

        let r = curly_bracket(&line)?;
        if r.status {
            if let Some(mut c) = r.content {
                tokens.append(&mut c);
            }
            continue;
        }

        let r = control_flow(&line)?;
        if r.status {
            if let Some(mut c) = r.content {
                tokens.append(&mut c);
            }
            continue;
        }

        // If we couldn't match this line then we know that it is a syntax error
        return Err(TokenizerError::new(format!(
            "Invalid syntax on line {}: {}",
            i + 1,
            tokenstream_display(&line)
        )));
    }

    Ok(tokens)
}
