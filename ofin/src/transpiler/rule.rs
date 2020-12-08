use super::OfinError;
use regex::Regex;
use std::fmt;

/// A block rule.
///
/// # Examples
///
/// ```ignore
/// let rule = BlockRule::new("b");
/// assert_eq!(rule.matches("abc").is_ok(), true);
/// ```
pub struct BlockRule {
    matcher: Regex,
    error: OfinError,
}

impl BlockRule {
    /// Create a new block rule.
    ///
    /// # Arguments
    ///
    /// * `matcher` - The regex matcher for this rule
    /// * `error` - The error message to provide in the event of a non-match
    pub fn new(matcher: &str, error: OfinError) -> Self {
        BlockRule {
            matcher: Regex::new(matcher).unwrap(),
            error,
        }
    }

    /// Check if some text matches this rule
    ///
    /// This will return `Ok()` if the text does not match,
    /// otherwise it will return an `Err()` with the supplied error variant.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to check
    pub fn matches<S: AsRef<str>>(&self, text: S) -> Result<(), OfinError> {
        if self.matcher.is_match(&text.as_ref()) {
            Err(self.error)
        } else {
            Ok(())
        }
    }
}

impl fmt::Debug for BlockRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.matcher, self.error,)
    }
}
