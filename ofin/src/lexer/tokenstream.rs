use super::Token;
use std::fmt;

pub struct TokenStream {
    inner: Vec<Token>,
}

impl TokenStream {
    /// Create a new token stream
    pub fn new() -> Self {
        TokenStream { inner: Vec::new() }
    }

    /// Append a token to the token stream
    ///
    /// # Arguments
    ///
    /// * `token` - The token to append
    pub fn push(&mut self, token: Token) {
        self.inner.push(token);
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl fmt::Debug for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.inner)
    }
}
