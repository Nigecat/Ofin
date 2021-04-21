use crate::utils::find_end_at;

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

    // ---------- Keywords ----------
    Using,

    // ---------- Other ----------
    Semicolon,
}

/// A token which contains both the type of the token (`t`) and the literal value of the token (`s`)
#[derive(Debug)]
pub struct Token {
    t: TokenType,
    s: String,
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
        )*
    };

    ($source: ident, $tokens: ident, [ $($t: tt => $s: literal),*, ]) => {
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
        )*
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
                Semicolon => ";",
                Equals => "=",

                Using => "using",
            ]);

            register!(source, tokens, [
                Target => ("<"..">" | include = false),
            ]);

            // Check for identifiers: a collection of [ascii] alphabetical characters
            if source.chars().next().map(|c| c.is_ascii_alphabetic()) == Some(true) {
                if let Some(s) = source.find(|c: char| !c.is_ascii_alphabetic()) {
                    let potential = source[0..s].to_string();

                    tokens.push(Token {
                        t: Ident,
                        s: potential.to_string(),
                    });

                    source.drain(0..potential.len());
                    continue;
                }
            }

            // Check for integers: a collection of 0-9 characters

            // Check for strings: a collection of text between two quotes

            // If we make it this far then we could not match the token
            panic!("invalid token");
        }

        TokenStream(tokens)
    }
}
