use crate::token::TokenType;
use lazy_static::lazy_static;

enum Regex {
    Primary(regex::Regex),
    Fallback(fancy_regex::Regex),
}

/// A syntax rule
pub struct Rule(Regex);

impl Rule {
    /// Find the rule in the supplied text, returns the (start, end) index of the match
    pub fn find(&self, text: &str) -> Option<(usize, usize)> {
        match &self.0 {
            Regex::Primary(r) => {
                let re = r.find(text)?;
                Some((re.start(), re.end()))
            }
            Regex::Fallback(r) => {
                let re = r.find(text).unwrap()?;
                Some((re.start(), re.end()))
            }
        }
    }
}

macro_rules! regex {
    ($t: expr, $re: literal) => {{
        // FIXME Add unit tests to ensure all regular expressions only match the start of strings
        let expr = format!("^{}", $re);
        if let Ok(r) = regex::Regex::new(&expr) {
            (Rule(Regex::Primary(r)), $t)
        } else {
            let r = fancy_regex::Regex::new(&expr).unwrap();
            (Rule(Regex::Fallback(r)), $t)
        }
    }};
}

/// Token types which don't mean anything and can be safetly removed
pub static SYNTAX_CLEAN: &[TokenType] = &[TokenType::Comment, TokenType::Space, TokenType::Control];

lazy_static! {
    /// The syntax rules for ofin.
    ///
    /// Note that these should be checked in their original order.
    pub static ref SYNTAX: Vec<(Rule, TokenType)> = vec![
        regex!(TokenType::Space, " "),
        regex!(TokenType::Eol, ";"),
        regex!(TokenType::Control, "[\r\n\t]"),
        regex!(TokenType::Comment, "//.*?(?=(\r?\n|$))"),

        regex!(TokenType::LBracket, r#"\("#),
        regex!(TokenType::RBracket, r#"\)"#),

        regex!(TokenType::Target, "<.*?>"),
        regex!(TokenType::Import, "import"),
        regex!(TokenType::Using, "using"),

        regex!(TokenType::Number, r#"\d+"#),
        regex!(TokenType::Ident, r#"\w+"#),
    ];
}
