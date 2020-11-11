use super::super::{clean, tokenstream_to_string, TokenStreamToken};
use super::{LineResult, LineResultType};
use crate::application::Application;
use crate::errors::TokenizerError;
use crate::symbols::{self, Symbol};
use crate::tokens::Token;

/// Handle variable declarations in the format `<type> <name> = <value>`
pub fn declare_variable(
    line: &[TokenStreamToken],
    application: &mut Application,
) -> LineResultType {
    // Int
    if line.len() > 3 && line[0] == 'i' && line[1] == 'n' && line[2] == 't' {
        trace!("Detected int declaration");
        // Get the position of the equals sign
        let eq_pos = line.iter().position(|c| c == Token::Equals);
        // If there is an equals sign
        if let Some(eq_pos) = eq_pos {
            // Extract the name of the variable
            let name = tokenstream_to_string(&clean(line[3..eq_pos].to_vec()))?;
            // Extract the value of the variable
            let value = tokenstream_to_string(&clean(line[eq_pos + 1..].to_vec()))?;
            trace!("Declaring variable '{}' with value: {}", name, value);
            let symbol = symbols::SInt::new(&value).ok_or_else(|| {
                TokenizerError::new(format!("'{}' is not a valid integer", value))
            })?;

            application.symbols.set(&name, Box::new(symbol));

            return Ok(LineResult::new(true, None));
        }
    }

    // String
    if line.len() > 6
        && line[0] == 's'
        && line[1] == 't'
        && line[2] == 'r'
        && line[3] == 'i'
        && line[4] == 'n'
        && line[5] == 'g'
    {
        trace!("Detected string declaration");
        // Get the position of the equals sign
        let eq_pos = line.iter().position(|c| c == Token::Equals);

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
            if value.first().unwrap() == Token::DoubleQuote
                && value.last().unwrap() == Token::DoubleQuote
            {
                // Remove the quotes
                value.remove(0);
                value.remove(value.len() - 1);
            } else {
                return Err(TokenizerError::new(
                    "String assignment must start and end with double quotes",
                ));
            }
            let value = tokenstream_to_string(&value)?;

            trace!("Declaring variable '{}' with value: {}", name, value);
            let symbol = symbols::SString::new(&value).unwrap();
            application.symbols.set(&name, Box::new(symbol));

            return Ok(LineResult::new(true, None));
        }
    }

    Ok(LineResult::new(false, None))
}
