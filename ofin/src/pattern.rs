use regex::Regex;

/// A transpile pattern.
///
/// # Examples
///
/// ```
/// use ofin::pattern::TranspilePattern;
///
/// let pattern = TranspilePattern::new("b", "c", None);
/// assert_eq!(pattern.replace("abc"), "acc");
/// ```
///
///
/// Using a backreference:
/// This takes the replace string `a`, finds it in the source text, then replaces it with the pattern `$1`.
/// Which is then pulled from the **1**st extractor, `b` which matches the string literal 'b' and uses it as the replacement.
/// ```
/// use ofin::pattern::TranspilePattern;
///
/// let pattern = TranspilePattern::new("a", "$1", Some(&["b"]));
/// assert_eq!(pattern.replace("abc"), "bbc");
/// ```
pub struct TranspilePattern {
    replace: Regex,
    with: String,
    extractors: Vec<Regex>,
}

impl<'t> TranspilePattern {
    /// Create a new transpile pattern.
    ///
    /// # Arguments
    ///
    /// * `replace` - The target text to replace
    /// * `with` - The text to replace the target with
    /// * `extractors` - An array of extractors to use for backreferencing
    pub fn new<S: AsRef<str>>(
        replace: &'t str,
        with: S,
        extractors: Option<&'t [&'t str]>,
    ) -> Self {
        TranspilePattern {
            replace: Regex::new(replace).unwrap(),
            with: with.as_ref().to_string(),
            extractors: extractors
                .unwrap_or(&[])
                .iter()
                .map(|e| Regex::new(e).unwrap())
                .collect(),
        }
    }

    /// Replace some text with this transpile pattern, returns the replaced text
    ///
    /// # Arguments
    ///
    /// * `text` - The text to run the replace on
    pub fn replace<S: AsRef<str>>(&self, text: S) -> String {
        let mut with = self.with.clone();

        for (i, extractor) in self.extractors.iter().enumerate() {
            let sign = format!("${}", i + 1);
            with = with.replace(&sign, &extractor.captures(&text.as_ref()).unwrap()[0]);
        }

        self.replace
            .replace_all(&text.as_ref(), with.as_str())
            .to_string()
    }
}
