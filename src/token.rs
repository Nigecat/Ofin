use crate::utils::find_end_at;
use std::ops::{Range, RangeBounds};

/// The type of a token
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TokenType {
    // ---------- Literals ----------
    Ident,
    Target,

    String,
    Integer,

    // ---------- Operators ----------
    Equals,
    Sub,
    Add,
    Mul,
    Div,

    // ---------- Keywords ----------
    Using,

    // ---------- Other ----------
    Semicolon,
}

/// A token which contains both the type of the token (`t`) and the literal value of the token (`s`)
#[derive(Debug)]
pub struct Token {
    pub t: TokenType,
    pub s: String,
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        &self.t == other
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
/// This method should be used on tokens which have a defined start and end point.
/// `include` states whether to include the beginning and end in the matched token.
/// ```rust
/// register!(source, tokens, [ TokenType::Target => ("<"..">" | include = false) ]);
/// ```
/// (this example matches `<.*>` and removes the `<>` from the token output)
///
/// ## Duration Tokens
/// This method should be used on tokens which have a range of characters which all match the same predicate
/// ```rust
/// register!(source, tokens, [ Ident => |c: char| c.is_ascii_alphabetic() ]);
/// ```
macro_rules! register {
    ($source: ident, [ $($c: literal),* $(,)? ]) => {
        $(
            if $source.starts_with($c) {
                $source.drain(0..$c.len());
                continue;
            }
        )*
    };

    ($source: ident, $tokens: ident, [ $($t: tt => $s: literal),* $(,)? ]) => {
        $(
            if $source.starts_with($s) {
                $source.drain(0..$s.len());
                $tokens.push(Token {
                    t: $t,
                    s: ::std::string::String::from($s),
                });
                continue;
            }
        )*
    };

    ($source: ident, $tokens: ident, [ $($t: tt => ($start: literal..$end: literal | include = $include: expr)),* $(,)? ]) => {
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
        )*
    };

    ($source: ident, $tokens: ident, [ $($t: tt => $predicate: expr),* $(,)? ]) => {
        $(
            if $source.chars().next().map($predicate) == Some(true) {
                let s = $source.find(|c: char| !($predicate)(c)).unwrap_or_else(|| $source.len());
                let potential = $source[0..s].to_string();

                $tokens.push(Token {
                    t: $t,
                    s: potential.to_string(),
                });

                $source.drain(0..potential.len());
                continue;
            }
        )*
    };
}

/// A stream of tokens
///
/// This is a wrapper around `Vec<Token>`.
#[derive(Debug)]
pub struct TokenStream(Vec<Token>);

impl TokenStream {
    pub fn position(&self, token: TokenType) -> Option<usize> {
        self.0.iter().position(|t| t == &token)
    }

    pub fn slice(&self, range: Range<usize>) -> &[Token] {
        &self.0[range]
    }

    pub fn rem<R: RangeBounds<usize>>(&mut self, range: R) -> Vec<Token> {
        self.0.drain(range).collect()
    }

    pub fn get(&self, index: usize) -> &Token {
        &self.0[index]
    }

    pub fn remove(&mut self, index: usize) -> Token {
        self.0.remove(index)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn lex(mut source: String) -> Result<Self, usize> {
        use TokenType::*;

        let original_length = source.len();
        let mut tokens = Vec::new();

        while !source.is_empty() {
            register!(source, [" ", "\r", "\n", "\t"]);

            register!(source, tokens, [
                Semicolon => ";",
                Equals => "=",
                Add => "+",
                Sub => "-",
                Div => "/",
                Mul => "*",

                Using => "using",
            ]);

            register!(source, tokens, [
                Target => ("<"..">" | include = false),

                String => ("\"".."\"" | include = false),
                String => ("'".."'" | include = false),
            ]);

            register!(source, tokens, [
                Ident => |c: char| c.is_ascii_alphabetic(),
                Integer => |c: char| c.is_ascii_digit(),
            ]);

            // If we make it this far then we could not match the token
            // We then return the byte index that we are currently up to so the caller knows where the parsing failed
            return Err(original_length - source.len());
        }

        // Combine any negative symbols before numbers ([TokenType::Sub, TokenType::Integer] -> -TokenType::Integer)
        let mut removed = 0;
        for mut i in 0..tokens.len() - 1 {
            i -= removed;
            if tokens[i] == Sub && tokens[i + 1] == Integer {
                removed += 1;
                tokens[i + 1] = Token {
                    t: Integer,
                    s: format!("-{}", tokens[i + 1].s),
                };
                tokens.remove(i);
            }
        }

        Ok(TokenStream(tokens))
    }
}
