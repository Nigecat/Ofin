use super::TokenType;
use regex::Regex;

/// Helper macro to create a regex and the identifying token
/// This automatically makes the regex only match the start of the line
macro_rules! regex {
    ($re: literal, $t: expr) => {
        (Regex::new(&format!("^{}", $re)).unwrap(), $t)
    };
}

lazy_static! {
    /// The token matchers
    static ref MATCHERS: Vec<(Regex, TokenType)> = vec![

    ];
}

pub struct TokenMatcher;

pub struct MatchResult {
    /// The type of token
    pub t: TokenType,
    /// The length of the token
    pub l: usize,
}

impl TokenMatcher {
    /// Figure out what token type a string starts with
    pub fn find<S: AsRef<str>>(s: S) -> Option<MatchResult> {
        let s = s.as_ref();

        for matcher in MATCHERS.iter() {
            let (r, t) = matcher;
            if let Some(mat) = r.find(s) {
                return Some(MatchResult {
                    t: *t,
                    l: mat.end(),
                });
            }
        }

        None
    }
}
