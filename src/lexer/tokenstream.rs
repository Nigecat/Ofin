use super::Token;
use std::fmt;

pub struct TokenStream<'t> {
    inner: Vec<Token<'t>>,
}

impl<'t> TokenStream<'t> {
    /// Create a new token stream
    pub fn new() -> Self {
        TokenStream { inner: Vec::new() }
    }

    /// Append a token to the token stream
    ///
    /// # Arguments
    ///
    /// * `token` - The token to append
    pub fn push(&mut self, token: Token<'t>) {
        self.inner.push(token);
    }
}

impl<'t> IntoIterator for TokenStream<'t> {
    type Item = Token<'t>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl fmt::Debug for TokenStream<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.inner)
    }
}
