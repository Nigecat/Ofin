use regex::Regex;

/// A transpile pattern for the transpiler
pub struct TranspilePattern<'t> {
    replace: Regex,
    with: &'t str,
}

impl<'t> TranspilePattern<'t> {
    /// Create a new transpile pattern
    ///
    /// # Arguments
    ///
    /// * `replace` - The target text to replace
    /// * `with` - The text to replace the target with
    pub fn new(replace: &'t str, with: &'t str) -> Self {
        TranspilePattern {
            replace: Regex::new(replace).unwrap(),
            with,
        }
    }

    /// Replace some text with this transpile pattern, returns the replaced text
    ///
    /// # Arguments
    ///
    /// * `text` - The text to run the replace on
    pub fn replace<S: AsRef<str>>(&self, text: S) -> String {
        self.replace
            .replace_all(&text.as_ref(), self.with)
            .to_string()
    }
}
