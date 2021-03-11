use super::Token;

pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn lex(source: String) -> Self {
        TokenStream { tokens: Vec::new() }
    }
}

impl IntoIterator for TokenStream {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}
