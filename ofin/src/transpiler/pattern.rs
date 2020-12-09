use pcre2::bytes::Regex;
use std::{fmt, str};

/// A transpile pattern.
///
/// # Examples
///
/// ```ignore
/// let pattern = TranspilePattern::new("b", "c", None, None);
/// assert_eq!(pattern.replace("abc"), "acc");
/// ```
///
///
/// Using a backreference:
/// This takes the replace string `a`, finds it in the source text, then replaces it with the pattern `$1`.
/// Which is then pulled from the  extractor, `b` which matches the string literal 'b' and uses it as the replacement.
/// ```ignore
/// let pattern = TranspilePattern::new("a", "$1", Some("b"), None);
/// assert_eq!(pattern.replace("abc"), "bbc");
/// ```
pub struct TranspilePattern {
    replace: Regex,
    with: String,
    extractor: Option<Regex>,
    mutator: Option<fn(&str) -> String>,
}

impl<'t> TranspilePattern {
    /// Create a new transpile pattern.
    ///
    /// # Arguments
    ///
    /// * `replace` - The target text to replace
    /// * `with` - The text to replace the target with
    /// * `extractor` - An extractor to use for backreferencing
    /// * `mutator` - An (optional) closure to mutate extractor values
    pub fn new<S: AsRef<str>>(
        replace: &'t str,
        with: S,
        extractor: Option<&'t str>,
        mutator: Option<fn(&str) -> String>,
    ) -> Self {
        TranspilePattern {
            replace: Regex::new(replace).unwrap(),
            with: with.as_ref().to_string(),
            extractor: extractor.and_then(|e| Some(Regex::new(e).unwrap())),
            mutator,
        }
    }

    /// Replace some text with this transpile pattern, returns the replaced text
    ///
    /// # Arguments
    ///
    /// * `text` - The text to run the replace on
    pub fn replace<S: AsRef<str>>(&self, text: S) -> String {
        let text = text.as_ref().to_string();
        let mut mut_text = text.clone();
        let mut with = self.with.clone();

        for capture in self.replace.find_iter(&text.as_bytes()) {
            let capture = capture.unwrap();
            let capture = &text[capture.start()..capture.end()];

            if let Some(extractor) = &self.extractor {
                if let Some(value) = extractor.captures(&text.as_bytes()).unwrap() {
                    let mut replace = str::from_utf8(value.get(0).unwrap().as_bytes())
                        .unwrap()
                        .to_string();
                    if let Some(mutator) = &self.mutator {
                        replace = mutator(&replace);
                    }
                    with = with.replace("$1", &replace);
                }
            }

            mut_text = mut_text.replace(capture, &with);
        }

        mut_text
    }
}

impl fmt::Debug for TranspilePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} -> {:?} -> {:?}",
            self.extractor, self.replace, self.with,
        )
    }
}
