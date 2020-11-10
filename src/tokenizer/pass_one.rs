use super::{TokenStream, TokenStreamReturn, TokenStreamToken};
use crate::tokens::Token;
use std::convert::TryFrom;

/// Process single character tokens (and ==)
pub fn pass_one(input: &str) -> TokenStreamReturn {
    debug!("Running pass 1 of tokenizer");

    // Whether we are currently inside quotes, this only affects double quotes
    let mut in_quotes = false;

    // Whether the next character should be escaped
    let mut escaped = false;

    let mut tokenstream: TokenStream = Vec::new();

    for c in input.chars() {
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
