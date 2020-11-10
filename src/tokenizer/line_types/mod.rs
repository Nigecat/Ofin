use crate::errors::TokenizerError;
mod declare_variable;
pub use declare_variable::declare_variable;

// If this returns a TokenizerError then it means that this line is not a match
// The bool is the status and if it is `false` then that indicates a syntax error in this line
type LineResult = Result<bool, TokenizerError>;
