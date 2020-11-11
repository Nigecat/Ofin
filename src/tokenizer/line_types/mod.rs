use crate::errors::TokenizerError;
use crate::tokens::Token;
mod control_flow;
mod curly_bracket;
mod declare_variable;
pub use control_flow::control_flow;
pub use curly_bracket::curly_bracket;
pub use declare_variable::declare_variable;

pub struct LineResult {
    /// If this is `false` then this line does not match the incoming tokenstream.
    /// If it is true then it does.
    pub status: bool,
    /// An optional return for the formatted tokens for this line, this should be `None` if this line does not match
    pub content: Option<Vec<Token>>,
}

impl LineResult {
    pub fn new(status: bool, content: Option<Vec<Token>>) -> Self {
        LineResult { status, content }
    }
}

// If this returns a TokenizerError then it means that this line has a syntax error in it
// The bool is the status and if it is `false` then that indicates this line doesn't match the incoming line
type LineResultType = Result<LineResult, TokenizerError>;
