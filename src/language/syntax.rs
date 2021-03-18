use crate::token::TokenType;
use lazy_static::lazy_static;
use regex::Regex;

macro_rules! regex {
    ($t: expr, $re: literal) => {
        (Regex::new(&format!(r#"^{}"#, $re)).unwrap(), $t)
    };
}

lazy_static! {
    /// The syntax rules for ofin.
    ///
    /// Note that these should be checked in their original order.
    pub static ref SYNTAX: Vec<(Regex, TokenType)> = vec![
        regex!(TokenType::Space, " "),
        regex!(TokenType::Eol, ";"),
        regex!(TokenType::Control, "[\r\n\t]"),

        regex!(TokenType::Target, "<.*?>"),
        regex!(TokenType::Import, "import"),
    ];
}
