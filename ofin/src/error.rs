use pcre2::bytes::Regex;
use std::fmt;
use thiserror::Error;

#[derive(PartialEq, Debug, Error)]
pub enum OfinError {
    #[error("This is an internal error and should not have occured")]
    InternalError,
    #[error("A syntax error has occured at row {row}, column {column}:\n{ctx}")]
    SyntaxError {
        /// The column this error occured in
        column: usize,
        /// The row this error occured in
        row: usize,
        /// Any context for this error
        ctx: String,
    },
    #[error("{ctx}:\n{row} | {code}")]
    SyntaxErrorV2 {
        /// The row this error occured in
        row: usize,
        /// The code that triggered this error
        code: String,
        /// The context of this error
        ctx: String,
    },
    #[error("{errors}")]
    Multi { errors: ErrorStream },
}

impl OfinError {
    /// Parse the output from rustc into our own error type
    ///
    /// # Arguments
    ///
    /// * `output` - The output from rustc
    ///
    /// # Returns
    ///
    /// The (row, ctx) of each error
    pub fn from_rustc(output: &str) -> Vec<(usize, String)> {
        let mut errors = Vec::new();
        let matcher =
            Regex::new(r#"error(?:\[.*?\]):(.*)\n[\S\s]*?\|\n(\d+)\s+\|\s+(.*)\n"#).unwrap();

        for capture in matcher.captures_iter(output.as_bytes()) {
            let error = capture.unwrap();
            let err_msg = error
                .get(1)
                .expect(&format!("unable to parse error message: {}", output));
            let err_line = error
                .get(2)
                .expect(&format!("unable to parse error message: {}", output));
            let err_msg = &output[err_msg.start()..err_msg.end()];
            // This comes from the regex pattern `\d+` so it should be impossible for it to be an invalid number
            let err_line = &output[err_line.start()..err_line.end()]
                .parse::<usize>()
                .unwrap()
                // Subtract the offset from the boilerplate code
                - crate::transpiler::OFFSET;

            errors.push((err_line, err_msg.to_string()));
        }

        errors
    }
}

/// A vector of errors
#[derive(PartialEq, Debug)]
pub struct ErrorStream {
    inner: Vec<OfinError>,
}

impl ErrorStream {
    pub fn new() -> Self {
        ErrorStream { inner: Vec::new() }
    }

    pub fn push(&mut self, error: OfinError) {
        self.inner.push(error);
    }
}

impl From<Vec<OfinError>> for ErrorStream {
    fn from(v: Vec<OfinError>) -> Self {
        ErrorStream { inner: v }
    }
}

impl fmt::Display for ErrorStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();

        for error in &self.inner {
            out.push_str(&format!("\n{}\n", error));
        }

        write!(f, "{}", out)
    }
}
