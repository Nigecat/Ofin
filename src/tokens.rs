use std::fmt::Display;

#[allow(dead_code)]
#[derive(Display, Debug, PartialEq)]
pub enum Token {
    /// An end of line ';'
    EOL,
    /// This token should never be found in the token stream, it is used as an impossible value token
    Infallible,
}
