use crate::utils::find_end_at;
use std::fmt;

// #[deny(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenType {
    // ---------- Literals ----------
    Target,

    // ---------- Keywords ----------
    Using,
}

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
