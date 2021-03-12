use super::def::TOKEN_MATCHERS;
use super::{Token, TokenType};
use crate::OfinError;

#[derive(Debug)]
pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    #[allow(mutable_borrow_reservation_conflict)] // TODO: Refactor in a way that avoids this
    #[trace::trace]
    pub fn lex(mut source: String) -> Result<Self, OfinError> {
        let mut tokens = Vec::new();

        while !source.is_empty() {
            let mut set = false;

            for (re, t) in TOKEN_MATCHERS.iter() {
                if let Some(mat) = re.find(&source) {
                    // This should only match the start of the string, if it matches any more then there is a bug in the regex
                    assert_eq!(mat.start(), 0);

                    let value: String = source.drain(0..mat.end()).collect();
                    tokens.push(Token { t: *t, value });
                    set = true;
                    break;
                }
            }

            // If we get here then none of the matchers matched
            // This means that there is a syntax error in the source
            if !set {
                return Err(OfinError::SynaxError(
                    source.lines().next().unwrap().to_string(),
                ));
            }
        }

        Ok(TokenStream { tokens })
    }

    /// Remove any tokens of type `t` from the stream
    #[trace::trace]
    pub fn filter(&mut self, t: TokenType) {
        self.tokens.retain(|token| *token != t);
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}
