use super::{clean, tokenstream_to_string, TokenStream, TokenStreamToken};
use crate::application::Application;
use crate::errors::TokenizerError;
use crate::symbols::{self, Symbol};
use crate::tokens::Token;

// If this returns a tokenizererror then it means that this line is not a match
// The bool is the status and if it is `false` then that indicates a syntax error in this line
type LineResult = Result<bool, TokenizerError>;

/// Handle variable declarations in the format `<type> <name> = <value>`
pub fn declare_variable(line: &TokenStream, application: &mut Application) -> LineResult {
    // Int
    if line.len() > 3
        && line[0] == TokenStreamToken::Char('i')
        && line[1] == TokenStreamToken::Char('n')
        && line[2] == TokenStreamToken::Char('t')
    {
        trace!("Detected int declaration");
        // Get the position of the equals sign
        let eq_pos = line
            .iter()
            .position(|c| c == &TokenStreamToken::Token(Token::Equals));
        // If there is no equals sign
        if let Some(eq_pos) = eq_pos {
            // Extract the name of the variable
            let name = tokenstream_to_string(&clean(line[3..eq_pos].to_vec()))?;
            // Extract the value of the variable
            let value = tokenstream_to_string(&clean(line[eq_pos + 1..].to_vec()))?;
            trace!("Declaring variable '{}' with value: {}", name, value);
            let symbol = symbols::SInt::new(&value).ok_or(TokenizerError::new(format!(
                "'{}' is not a valid integer",
                value
            )))?;

            application.symbols.set(&name, Box::new(symbol));

            return Ok(true);
        }
    }

    // String
    if line.len() > 6
        && line[0] == TokenStreamToken::Char('s')
        && line[1] == TokenStreamToken::Char('t')
        && line[2] == TokenStreamToken::Char('r')
        && line[3] == TokenStreamToken::Char('i')
        && line[4] == TokenStreamToken::Char('n')
        && line[5] == TokenStreamToken::Char('g')
    {
        trace!("Detected string declaration");
        return Ok(true);
    }

    Ok(false)
}
