use crate::utils::find_end_at;
use std::fmt;

/// The type of a token
#[deny(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenType {
    // ---------- Literals ----------
    Target,

    // ---------- Keywords ----------
    Using,
}

/// A token which contains both the type of the token (`t`) and the literal value of the token (`s`)
#[derive(Debug)]
pub struct Token {
    t: TokenType,
    s: String,
}

impl Token {
    fn new(t: TokenType, s: String) -> Self {
        Token { t, s }
    }
}

/// Register token rules
///
/// ## Token Removal
/// This method should be used on literals that bear no meaning and can be safely removed.
/// ```rust
/// register!(source, tokens, [ "\n" ]);
/// ```
///
/// ## Literal Tokens
/// This method should be used on tokens which have a static literal meaning (primarily keywords).
/// ```rust
/// register!(source, tokens, [ TokenType::Using => "using" ]);
/// ```
///
/// ## Start/End Tokens
/// This methods should be used on tokens which have a defined start and end point.
/// `include` states whether to include the beginning and end in the matched token.
/// ```rust
/// register!(source, tokens, [ TokenType::Target => ("<"..">" | include = false) ]);
/// ```
/// (this example matches `<.*>` and removes the `<>` from the token output)
macro_rules! register {
    ($source: ident, [ $($c: literal),*, ]) => {
        $(
            if $source.starts_with($c) {
                $source.drain(0..$c.len());
                continue;
            }
        ),*
    };

    ($source: ident, $tokens: ident, [ $($t: tt => $s: literal),*, ]) => {
        $(
            if $source.starts_with($s) {
                $source.drain(0..$s.len());
                $tokens.push(Token {
                    t: $t,
                    s: String::from($s),
                });
                continue;
            }
        ),*
    };

    ($source: ident, $tokens: ident, [ $($t: tt => ($start: literal..$end: literal | include = $include: expr)),*, ]) => {
        $(
            if $source.starts_with($start) {
                if let Some(end) = find_end_at(&$source, $start.len(), $end) {
                    if $include {
                        $tokens.push(Token {
                            t: $t,
                            s: $source[0..end].to_string(),
                        });
                    } else {
                        $tokens.push(Token {
                            t: $t,
                            s: $source[$start.len()..end - $end.len()].to_string(),
                        });
                    }

                    $source.drain(0..end);
                    continue;
                }
            }
        ),*
    };
}

/// A stream of tokens
///
/// This is a wrapper around `Vec<Token>`.
#[derive(Debug)]
pub struct TokenStream(Vec<Token>);

impl TokenStream {
    pub fn lex(mut source: String) -> Self {
        use TokenType::*;

        let mut tokens = Vec::new();

        while !source.is_empty() {
            register!(source, [" ",]);

            register!(source, tokens, [
                Using => "using",
            ]);

            register!(source, tokens, [
                Target => ("<"..">" | include = false),
            ]);

            // If we make it this far then we could not match the token
            panic!("invalid token");
        }

        TokenStream(tokens)
    }
}
