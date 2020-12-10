use ofin_derive::VariantCount;
use pcre2::bytes::Regex;
use std::{fmt, str};

#[derive(Clone, Copy, VariantCount, PartialEq, Debug)]
pub enum TokenType {
    /// "A String Literal"
    StringLiteral,
    /// \r?\n
    NewLine,
    /// ;
    EOL,
}

pub struct Token {
    token: TokenType,
    length: usize,
    literal: String,
}

/// Create a new regex
macro_rules! regex {
    ($regex:expr) => {
        Regex::new($regex).unwrap()
    };
}

lazy_static! {
    /// A static array with instructions on how to match each type of token
    static ref TOKEN_MATCHERS: [(Regex, TokenType); TokenType::VARIANT_COUNT] = [
        (regex!(r#"(")((?:(?!")[^\\]|(?:\\\\)*\\[^\\])*)(")"#), TokenType::StringLiteral),
        (regex!(r#"\r?\n"#), TokenType::NewLine),
        (regex!(r#";"#), TokenType::EOL),
    ];
}

impl Token {
    /// Attempt to convert the start of a string into a token
    pub fn try_from_str_start(string: &str) -> Option<Self> {
        for (matcher, token) in TOKEN_MATCHERS.iter() {
            trace!("Checking matcher for {:?}: {:?}", token, matcher);

            if let Ok(captures) = matcher.captures(string.as_bytes()) {
                if let Some(captures) = &captures {
                    let text = &captures[0];
                    let text = str::from_utf8(&text).unwrap();
                    // Check if the left-most match is the same as the start of the string
                    if text == &string[..text.len()] {
                        trace!("Detected token: {:?}", token);
                        return Some(Token {
                            token: *token,
                            literal: text.to_string(),
                            length: text.len(),
                        });
                    }
                }
            }
        }

        // If an early return wasn't trigger by the matchers,
        // we can safetly assume this string does not start with a valid token
        trace!("Unable to match token");
        None
    }

    pub fn token(&self) -> TokenType {
        self.token
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn literal(&self) -> &str {
        &self.literal
    }
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        match token.token {
            TokenType::StringLiteral => format!("OfinString::new({})", token.literal()),
            _ => token.literal().to_string(),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.token, self.literal)
    }
}
