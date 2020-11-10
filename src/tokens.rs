use std::fmt::Display;

#[allow(dead_code)]
#[derive(Display, Debug, PartialEq, Eq, Clone)]
pub enum Token {
    /// This token should never be found in the token stream, it is used as an impossible value token
    Infallible,
    /// An end of line ';'
    EOL,
    /// {
    LCurly,
    /// }
    RCurly,
    /// "
    DoubleQuote,
    /// A symbol, this should store the name of the symbol, and should match up to the symbol table
    Symbol(String),
    /// A constant, this should store the index of the constant in the constant table
    Constant(u32),
}
