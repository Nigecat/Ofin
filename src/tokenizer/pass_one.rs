use super::{TokenStream, TokenStreamToken};
use crate::tokens::Token;
use crate::errors::TokenizerError;
use std::convert::TryFrom;

/// Process single character tokens (and ==)
pub fn pass_one(input: &str) -> Result<Vec<TokenStreamToken>, TokenizerError> {
    debug!("Running pass 1 of tokenizer");

    // Whether we are currently inside quotes, this only affects double quotes
    let mut in_quotes = false;

    // Whether the next character should be escaped
    let mut escaped = false;

    // Whether we are currently inside a single line comment (// )
    let mut in_comment = false;

    // Whether we are current;y inside a block comment (/* */)
    let mut in_block_comment = false;

    let mut tokenstream: TokenStream = Vec::new();

    for (i, c) in input.chars().enumerate() {
        // If this is the end of a multi line comment
        if in_block_comment
            && input.chars().nth(i - 1).unwrap_or(' ') == '*'
            && c == '/'
        {
            in_block_comment = false;
            continue;
        }

        if in_comment && c == '\n' {
            in_comment = false;
            continue;
        }

        // If this is the start of a single line comment
        if c == '/' && input.chars().nth(i + 1).unwrap_or(' ') == '/' {
            in_comment = true;
            continue;
        }

        // If this is the start of a multi line comment
        if c == '/' && input.chars().nth(i + 1).unwrap_or(' ') == '*' {
            in_block_comment = true;
            continue;
        }

        if in_comment || in_block_comment {
            continue;
        }

        if in_quotes {
            if escaped {
                escaped = false;
                tokenstream.push(TokenStreamToken::Char(c));
                continue;
            }

            if c == '\\' {
                escaped = true;
                continue;
            }

            if c == '"' {
                in_quotes = false;
                tokenstream.push(TokenStreamToken::Token(Token::DoubleQuote));
                continue;
            }

            tokenstream.push(TokenStreamToken::Char(c));
            continue;
        }

        tokenstream.push(match c {
            '"' => {
                in_quotes = true;
                TokenStreamToken::Token(Token::DoubleQuote)
            }
            // Remove any control characters (by replacing them with a space)
            '\n' => TokenStreamToken::Char(' '),
            '\r' => TokenStreamToken::Char(' '),
            '\t' => TokenStreamToken::Char(' '),
            _ => {
                if let Ok(t) = Token::try_from(c) {
                    TokenStreamToken::Token(t)
                } else {
                    TokenStreamToken::Char(c)
                }
            }
        });
    }

    Ok(tokenstream)
}
