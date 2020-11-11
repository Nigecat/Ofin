use crate::errors::TokenizerError;
mod declare_variable;
pub use declare_variable::declare_variable;

// If this returns a TokenizerError then it means that this line has a syntax error in it
// The bool is the status and if it is `false` then that indicates this line doesn't match the incoming line
type LineResult = Result<bool, TokenizerError>;
