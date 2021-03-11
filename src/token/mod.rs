mod def;
mod stream;
pub use def::TokenType;
pub use stream::TokenStream;

#[derive(Debug)]
pub struct Token {
    /// The type of this token
    t: TokenType,
    /// The value of this token
    value: String,
}

impl Token {
    pub fn t(&self) -> TokenType {
        self.t
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
