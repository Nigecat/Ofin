use lazy_static::lazy_static;
use regex::Regex;

/// The different types of tokens
///
/// Unless otherwise specified, these match the same text as their variant name
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType {
    Import,

    /// ;
    Eol,

    /// Any text wrapped in <> (e.g <TEXT>)
    Target,

    /// A control character (e.g \r or \n)
    Control,

    /// U+0020
    Space,
}

/// This automatically makes the regex only match the start of the line
macro_rules! regex {
    ($t: expr, $re: literal) => {
        (Regex::new(&format!(r#"^{}"#, $re)).unwrap(), $t)
    };
}

lazy_static! {
    /// Note that these should be checked in order
    pub static ref TOKEN_MATCHERS: Vec<(Regex, TokenType)> = vec![
        regex!(TokenType::Space, " "),
        regex!(TokenType::Eol, ";"),
        regex!(TokenType::Control, "[\r\n\t]"),

        regex!(TokenType::Import, "import"),
        regex!(TokenType::Target, "<.*?>"),
    ];
}
