use std::fmt::Display;

#[allow(dead_code)]
#[derive(Display, Debug, PartialEq)]
pub enum Token {
    /// An end of line ';'
    EOL,
    /// This token should never be found in the token stream, it is used as an impossible value token
    Infallible,
    /// A symbol, this should store the name of the symbol, and should match up to the symbol table
    Symbol(String),
    /// A constant, this should store the index of the constant in the constant table
    Constant(u32),
}
