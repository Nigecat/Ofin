use pcre2::bytes::Regex;
use std::{fmt, str};

/// A token matcher.
///
/// This struct is used for matching strings to tokens.
pub struct TokenMatcher {
    name: String,
    matcher: Regex,
    replace_with: String,
    preprocess: bool,
    extractor: Option<Regex>,
    mutator: Option<fn(&str) -> String>,
}

impl TokenMatcher {
    /// Create a new token matcher
    ///
    /// # Arguments
    ///
    /// * `name` - The name of this token
    /// * `matcher` - A regex string to use for matching
    /// * `replace_with` - The text to replace the match with when converting back into a string
    /// * `preprocess` - Whether this matcher should be preprocessed before tokenizing
    /// * `extrator` - An (optional) regex string that will be matched against the result of the `matcher` regex then interpolated into a `$1` string inside the `replace_with` text
    /// * `mutator` - An (optional) closure to mutate the value returned from the extractor before interpolation
    pub fn new(
        name: String,
        matcher: String,
        replace_with: String,
        preprocess: bool,
        extractor: Option<String>,
        mutator: Option<fn(&str) -> String>,
    ) -> Self {
        TokenMatcher {
            name,
            matcher: Regex::new(&matcher).unwrap(),
            replace_with,
            preprocess,
            extractor: extractor.map(|e| Regex::new(&e).unwrap()),
            mutator,
        }
    }

    /// Get the name of the token this matcher matches
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if this matcher should be preprocessd
    pub fn should_preprocess(&self) -> bool {
        self.preprocess
    }

    /// Replace the matched contents of a string
    pub fn replace_str(&self, mut string: String) -> String {
        let source = string.clone();
        // Reverse the order of the matches
        // This is required since we get the byte range of the matched text in order from first-last
        // If we just replace them in that order we mutate the string and the following matches become
        //      invalid as the length of the string changes
        // However, since the iterator guarantees to be first to last,
        //      we can reverse the results to completely avoid that issue
        //      as the indexes are based off the string start not end
        let matches = self.matcher.find_iter(&source.as_bytes());
        let matches: Vec<Result<pcre2::bytes::Match, pcre2::Error>> = matches.collect();
        for result in matches.iter().rev() {
            if let Ok(mat) = result {
                let text = &string[mat.start()..mat.end()];

                let token = Token {
                    matcher: self,
                    token: &self.name,
                    length: text.len(),
                    literal: text.to_string(),
                };

                let to: String = token.into();
                debug!("{} -> {}", text, to);
                string = string.replace(text, &to);
            }
        }

        string
    }

    /// Attempt to convert the start of a string into a token
    pub fn try_from_str_start<S: AsRef<str>>(&self, string: S) -> Option<Token> {
        let string = string.as_ref();

        if let Ok(captures) = self.matcher.captures(string.as_bytes()) {
            if let Some(captures) = &captures {
                let text = &captures[0];
                let text = str::from_utf8(&text).unwrap();

                // Check if the left-most match is the same as the start of the string
                if string.starts_with(text) {
                    return Some(Token {
                        matcher: self,
                        token: &self.name,
                        length: text.len(),
                        literal: text.to_string(),
                    });
                }
            }
        }

        None
    }
}

/// A token
pub struct Token<'t> {
    matcher: &'t TokenMatcher,
    token: &'t str,
    length: usize,
    literal: String,
}

impl Token<'_> {
    /// Get the type (name) of this token
    pub fn token(&self) -> &str {
        self.token
    }

    /// Get the character length of this token
    pub fn length(&self) -> usize {
        self.length
    }
}

impl fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.token, self.literal)
    }
}

impl From<Token<'_>> for String {
    fn from(token: Token) -> Self {
        let mut text = token.matcher.replace_with.to_string();

        if let Some(extractor) = &token.matcher.extractor {
            if let Ok(extracted) = extractor.captures(token.literal.as_bytes()) {
                if let Some(extracted) = &extracted {
                    let extracted = &extracted[0];
                    let mut extracted = str::from_utf8(&extracted).unwrap().to_string();

                    if let Some(mutator) = token.matcher.mutator {
                        extracted = mutator(&extracted);
                    }

                    text = text.replace("$1", &extracted);
                }
            }
        }

        text
    }
}
