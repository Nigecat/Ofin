use crate::error::{Error, SyntaxError};
use crate::Source;
use std::convert::TryInto;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TokenType {
    // ---------- Single character tokens ----------
    Equals,
    Space,
    LCurly,
    RCurly,
    LBracket,
    RBracket,

    // ---------- 1-2 character tokens ----------

    // ---------- Multi character tokens ----------

    // ---------- Literals ----------
    Identifier,
    String,
    Number,

    // ---------- Keywords ----------
    And,
    Else,
    False,
    True,
    Func,
    If,
    Or,
    Return,
    While,

    // ---------- Control characters ----------
    Newline,
    Tab,
    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub t: TokenType,
    pub lexeme: String,
    pub line: usize,
}

pub fn lex(source: Source) -> Result<Vec<Token>, Error> {
    use TokenType::*;

    let mut source: ::std::string::String = source.try_into()?;
    let mut tokens = Vec::new();
    let mut line = 1;

    macro_rules! f {
        ($t: ident => $c: literal) => {
            if source.starts_with($c) {
                let lexeme = source.drain(..$c.len()).collect();
                tokens.push(Token {
                    t: $t,
                    lexeme,
                    line,
                });
                continue;
            }
        };

        ($t: ident => $predicate: tt) => {
            if $predicate(source.chars().next().unwrap()) {
                let end = source
                    .find(|c| !$predicate(c))
                    .unwrap_or_else(|| source.len());
                let lexeme = source.drain(..end).collect();
                tokens.push(Token {
                    t: $t,
                    lexeme,
                    line,
                });
                continue;
            }
        };
    }

    while !source.is_empty() {
        // Increment to the next line if our last token was a newline
        if tokens.last().map(|token: &Token| token.t) == Some(Newline) {
            line += 1;
        }

        f!(Space => " ");
        f!(Newline => "\n");
        f!(Tab => "\t");

        f!(Equals => "=");
        f!(LCurly => "{");
        f!(RCurly => "}");
        f!(LBracket => "(");
        f!(RBracket => ")");

        f!(And => "&&");
        f!(Or => "||");
        f!(Else => "else");
        f!(False => "false");
        f!(True => "true");
        f!(Func => "func");
        f!(While => "while");
        f!(If => "if");
        f!(Return => "return");

        f!(Number => (|c: char| c.is_numeric()));
        f!(Identifier => (|c: char| c.is_alphabetic()));

        // String
        if source.starts_with('"') {
            if let Some(end) = source.chars().skip(1).position(|c| c == '\n' || c == '"') {
                let end_char = source.chars().nth(end + 1).unwrap();
                if end_char == '"' {
                    let mut lexeme = source.drain(..end + 2).collect::<::std::string::String>();
                    lexeme.remove(0);
                    lexeme.pop();
                    tokens.push(Token {
                        t: TokenType::String,
                        lexeme,
                        line,
                    });
                    continue;
                }
            }
        }

        return Err(SyntaxError::new(
            line,
            "source",
            format!("unexpected token: '{}'", source.chars().next().unwrap()),
        )
        .into());
    }

    tokens.push(Token {
        t: TokenType::Eof,
        lexeme: Default::default(),
        line,
    });

    Ok(tokens)
}
