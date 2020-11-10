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

    let tokenstream: TokenStream = input
        .chars()
        .map(|c| {
            if in_quotes {
                if escaped {
                    escaped = false;
                    return TokenStreamToken::Char(c);
                }

                if c == '\\' {
                    escaped = true;
                    return TokenStreamToken::Char('\\');
                }

                if c == '"' {
                    in_quotes = false;
                    return TokenStreamToken::Token(Token::DoubleQuote);
                }
                return TokenStreamToken::Char(c);
            }

            match c {
                '"' => {
                    in_quotes = true;
                    TokenStreamToken::Token(Token::DoubleQuote)
                },
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
                },
            }
        })
        .collect();

    Ok(tokenstream)
}
