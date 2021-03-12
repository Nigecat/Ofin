mod def;
mod stream;
pub use def::TokenType;
use std::fmt;
pub use stream::TokenStream;

pub struct Token {
    /// The type of this token
    t: TokenType,
    /// The value of this token
    value: String,
}

impl Token {
    #[trace::trace]
    pub fn t(&self) -> TokenType {
        self.t
    }

    #[trace::trace]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, t: &TokenType) -> bool {
        self.t == *t
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.t, self.value)
    }
}
