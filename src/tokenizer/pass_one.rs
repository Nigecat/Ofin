use super::{TokenStream, TokenStreamReturn, TokenStreamToken};
use crate::tokens::Token;

pub fn pass_one(input: &str) -> TokenStreamReturn {
    debug!("Running pass 1 of tokenizer");

    // Whether we are currently inside quotes, this only affects double quotes
    let mut in_quotes = false;

    // Whether the next character should be escape
    let mut escaped = false;

    let tokenstream: TokenStream = input
        .chars()
        .map(|c| {
            if escaped {
                escaped = false;
                return TokenStreamToken::Char(c);
            }

            if in_quotes {
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
                ';' => TokenStreamToken::Token(Token::EOL),
                '{' => TokenStreamToken::Token(Token::LCurly),
                '}' => TokenStreamToken::Token(Token::RCurly),
                '"' => {
                    in_quotes = true;
                    TokenStreamToken::Token(Token::DoubleQuote)
                }
                _ => TokenStreamToken::Char(c),
            }
        })
        .collect();

    Ok(tokenstream)
}
