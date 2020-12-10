mod token;
mod tokenstream;
use crate::OfinError;
use std::str;
pub use token::{Token, TokenType};
pub use tokenstream::TokenStream;

/// Lex a string
///
/// # Arguments
///
/// * `source` - The string to convert into tokens
pub fn lex<S: AsRef<str>>(source: S) -> Result<TokenStream, OfinError> {
    let mut source = source.as_ref().to_string();
    let mut tokens = TokenStream::new();

    // The current row of the lexer in the source string, this is increased by one for every newline found
    let mut row = 1;

    // The current column (character index in this row) of the lexer in the source string,
    // this is reset to 1 every newline
    let mut column = 1;

    // While there are still characters left
    while !source.is_empty() {
        trace!("Running lexer on row {}, column {}", row, column);

        // Attempt to convert the start of the source string into a token
        // If it fails then report the line and column number
        let token = Token::try_from_str_start(&source);
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
        let token = token.unwrap();

        // Check if this is a newline token,
        // this is not the same as an end of line token (;) and is not included in the token output
        if token.token() == TokenType::NewLine {
            row += 1;
            column = 1;
        } else {
            // Increase the column by the length of this token
            column += token.length();
        }

        // Remove the length of this token from the start of the string
        source.drain(0..token.length());

        tokens.push(token);
    }

    Ok(tokens)
}
