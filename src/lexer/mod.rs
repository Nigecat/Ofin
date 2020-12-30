mod syntax;
mod token;
mod tokenstream;
use crate::OfinError;
use std::str;
use token::{Token, TokenMatcher};
pub use tokenstream::TokenStream;

lazy_static! {
    static ref TOKEN_MATCHERS: Vec<TokenMatcher> = syntax::syntax();
}

/// Lex a string
///
/// # Arguments
///
/// * `source` - The string to convert into tokens
pub fn lex<'t, S: AsRef<str>>(source: S) -> Result<TokenStream<'t>, OfinError> {
    let mut source = source.as_ref().to_string();
    let mut tokens = TokenStream::new();

    // The current row of the lexer in the source string, this is increased by one for every newline found
    let mut row = 1;

    // The current column (character index in this row) of the lexer in the source string,
    // this is reset to 1 every newline
    let mut column = 1;

    // Run preprocessed matchers
    for matcher in TOKEN_MATCHERS.iter().filter(|m| m.should_preprocess()) {
        debug!("Preprocessing: {}", matcher.name());
        source = matcher.replace_str(source);
    }

    // While there are still characters left
    while !source.is_empty() {
        debug!("Running lexer on row {}, column {}", row, column);

        // Check if any of the token matchers pass
        let token = TOKEN_MATCHERS
            .iter()
            .filter(|m| !m.should_preprocess())
            .find(|m| m.try_from_str_start(&source).is_some());

        if token.is_none() {
            return Err(OfinError::SyntaxError {
                column,
                row,
                ctx: {
                    let end_of_line = source
                        .chars()
                        .position(|c| c == '\n')
                        .unwrap_or_else(|| source.len());

                    format!(
                        "{}| {}{}",
                        row,
                        if column == 1 { "" } else { "..." },
                        &str::from_utf8(source.as_bytes()).unwrap()[..end_of_line]
                    )
                },
            });
        }

        // It is now safe to unwrap
        let token = token.unwrap().try_from_str_start(&source).unwrap();
        debug!(
            "Detected token {:?} with length {}",
            token.token(),
            token.length()
        );

        // Remove the length of this token from the start of the string
        source.drain(0..token.length());

        // Check if this is a newline token,
        // this is not the same as an end of line token (;)
        if token.token() == "Newline" {
            row += 1;
            column = 1;
        } else {
            // Increase the column by the length of this token
            column += token.length();
        }
        tokens.push(token);
    }

    Ok(tokens)
}