#[derive(Debug)]
pub struct Expression {
    tokens: Vec<Token>,
}

impl Expression {
    /// Parse the given input to an expression
    pub fn parse<S: AsRef<str>>(input: S) -> Self {
        todo!();
    }

    /// Peek the next token
    fn peek(&self) -> Option<Token> {
        self.tokens.last().copied()
    }

    /// Get the next token
    fn next(&mut self) -> Option<Token> {
        self.tokens.pop()
    }
}
