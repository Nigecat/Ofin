use super::TokenMatcher;
use ofin_derive::VariantCount;
use std::slice::Iter;

#[derive(Debug, PartialEq, VariantCount, Copy, Clone)]
pub enum TokenType {
    StringLiteral,
    Newline,
    EOL,
}

#[rustfmt::skip]
impl TokenType {
    pub fn iterator() -> Iter<'static, TokenMatcher> {
        lazy_static! {
            static ref TOKEN_MATCHERS: [TokenMatcher; TokenType::VARIANT_COUNT] = [
                TokenMatcher::new(TokenType::StringLiteral, r#"(")((?:(?!")[^\\]|(?:\\\\)*\\[^\\])*)(")"#, "OfinString::new($1)", Some(r#"(")((?:(?!")[^\\]|(?:\\\\)*\\[^\\])*)(")"#), None),
                TokenMatcher::new(TokenType::Newline, r#"\r?\n"#, "", None, None),
                TokenMatcher::new(TokenType::EOL, ";", ";", None, None),
            ];
        }
        TOKEN_MATCHERS.iter()
    }
}
