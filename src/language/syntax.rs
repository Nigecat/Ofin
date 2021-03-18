use crate::token::TokenType;
use lazy_static::lazy_static;

enum Regex {
    Primary(regex::Regex),
    Fallback(fancy_regex::Regex),
}

/// A syntax rule
pub struct Rule(Regex);

impl Rule {
    fn new(r: Regex) -> Self {
        Rule(r)
    }

    pub fn find(&self, text: &str) -> (usize, usize) {
        match self.0 {
            Regex::Primary(r) => r.find(text),
            Regex::Fallback(r) => r.find(text).unwrap(),
        }
    }
}

macro_rules! regex {
    ($t: expr, $re: literal) => {
        {
            let expr = format!("^{}", $re);
            if let Ok(r) = regex::Regex::new(&expr) {
                (Regex::Primary(r), $t)
            } else {
                let r = fancy_regex::Regex::new(&expr).unwrap();
                (Regex::Fallback(r), $t)
            }
        }
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
