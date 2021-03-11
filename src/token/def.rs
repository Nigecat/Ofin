use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    /// ;
    Eol,

    /// \r?\n
    Newline,
}

/// This automatically makes the regex only match the start of the line
macro_rules! regex {
    ($re: literal, $t: expr) => {
        (Regex::new(&format!(r#"^{}"#, $re)).unwrap(), $t)
    };
}

lazy_static! {
    pub static ref TOKEN_MATCHERS: Vec<(Regex, TokenType)> = vec![
        regex!(";", TokenType::Eol),
        regex!("\r?\n", TokenType::Newline),
    ];
}
