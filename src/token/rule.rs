use crate::token::TokenType;

enum RuleType {
    Plain(&'static str),
    Regex(regex::Regex),
    FancyRegex(fancy_regex::Regex),
}

pub struct Rule(RuleType, TokenType);

impl Rule {
    #[allow(const_item_mutation)]
    pub fn new(t: TokenType, s: &'static str) -> Self {
        lazy_static! {
            static ref CHARS: Vec<char> =
                "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
                    .chars()
                    .collect();
        }

        // If all characters fit in [a-zA-Z0-9] then assume it's a plaintext match
        let rule = if s.chars().all(|c| CHARS.contains(&c)) {
            RuleType::Plain(s)
        } else {
            // Otherwise attempt to init a normal regex and if that fails init a fancy one
            let re = &format!("^{}", s);
            match regex::Regex::new(re) {
                Ok(re) => RuleType::Regex(re),
                Err(_) => RuleType::FancyRegex(fancy_regex::Regex::new(re).unwrap()),
            }
        };

        Rule(rule, t)
    }

    /// Given a string and a rule find the position from 0..<usize> that the rule covers
    pub fn find(&self, s: &str) -> Option<usize> {
        match &self.0 {
            RuleType::Plain(p) => {
                if s.starts_with(p) {
                    Some(p.len())
                } else {
                    None
                }
            }
            RuleType::Regex(re) => re.find(s).map(|res| res.end()),
            RuleType::FancyRegex(re) => re.find(s).unwrap().map(|res| res.end()),
        }
    }

    pub fn t(&self) -> TokenType {
        self.1
    }
}
