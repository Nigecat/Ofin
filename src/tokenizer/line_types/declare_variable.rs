use super::super::{clean, tokenstream_to_string, TokenStream, TokenStreamToken};
use super::LineResult;
use crate::application::Application;
use crate::errors::TokenizerError;
use crate::symbols::{self, Symbol};
use crate::tokens::Token;

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
        // If there is an equals sign
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
        // Get the position of the equals sign
        let eq_pos = line
            .iter()
            .position(|c| c == &TokenStreamToken::Token(Token::Equals));

        // If there is an equals sign
        if let Some(eq_pos) = eq_pos {
            // Extract the name of the variable
            let name = tokenstream_to_string(&clean(line[6..eq_pos].to_vec()))?;
            // Extract the value of the variable
            let mut value = clean(line[eq_pos + 1..].to_vec());
            // Make sure the text is surrounded in quotes
            if value.len() < 2 {
                return Err(TokenizerError::new("Invalid string assignment"));
            }
            if value.first().unwrap() == &TokenStreamToken::Token(Token::DoubleQuote)
                && value.last().unwrap() == &TokenStreamToken::Token(Token::DoubleQuote)
            {
                // Remove the quotes
                value.remove(0);
                value.remove(value.len() - 1);
            } else {
                return Err(TokenizerError::new("String assignment must start and end with double quotes"));
            }
            let value = tokenstream_to_string(&value)?;

            trace!("Declaring variable '{}' with value: {}", name, value);
            let symbol = symbols::SString::new(&value).unwrap();
            application.symbols.set(&name, Box::new(symbol));

            return Ok(true);
        }
    }

    Ok(false)
}
