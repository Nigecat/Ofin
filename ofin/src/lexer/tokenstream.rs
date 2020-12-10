use super::Token;

#[derive(Debug)]
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
