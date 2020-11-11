use super::{tokenstream_to_string, TokenStream, TokenStreamToken};
use crate::application::Application;
use crate::errors::TokenizerError;
use crate::symbols::*;
use crate::tokens::{Expression, Token};
use crate::util::is_digit;
use std::convert::TryFrom;

/// Tokenize expressions
pub fn pass_two(
    tokenstream: TokenStream,
    application: &mut Application,
) -> Result<TokenStream, TokenizerError> {
    debug!("Running pass 2 of tokenizer");

    // Whether we are currently in an expression
    let mut in_expr = false;

    // The expression we are currently in
    let mut expr: Vec<TokenStreamToken> = Vec::new();

    let mut tokens: TokenStream = Vec::new();
    for t in tokenstream {
        if in_expr && &t == Token::RBracket {
            in_expr = false;

            // 'Expr' is now a vector of the TokenStreamToken(s) contained within the expression
            //-----------------------------------------------------------------------------------

            // We first take out any strings and put them in the constants table
            while expr.contains(&Token::DoubleQuote.into()) {
                let pos_start = expr.iter().position(|t| t == Token::DoubleQuote).unwrap();
                expr.remove(pos_start);
                let pos_end = expr
                    .iter()
                    .position(|t| t == Token::DoubleQuote)
                    .ok_or_else(|| {
                        TokenizerError::new("Missing ending quote in expression string literal")
                    })?;

                if pos_start != pos_end {
                    expr.drain(pos_start + 1..pos_end + 1);
                }

                let sym = application.constants.insert(Box::new(
                    SString::new(&tokenstream_to_string(&expr[pos_start..pos_end]).unwrap())
                        .unwrap(),
                ));

                expr[pos_start] = Token::Constant(sym).into();
            }

            // We then take out any integers and put them in the constant table
            while expr
                .iter()
                .any(|t| t.is_char() && is_digit(char::try_from(t).unwrap()))
            {
                let pos_start = expr
                    .iter()
                    .position(|t| t.is_char() && is_digit(char::try_from(t).unwrap()))
                    .unwrap();
                let pos_end = expr
                    .iter()
                    .skip(pos_start)
                    .position(|t| !(t.is_char() && is_digit(char::try_from(t).unwrap())))
                    //  .and_then(|p| Some(p - 1))
                    .unwrap_or_else(|| expr.len());

                let sym = application.constants.insert(Box::new(
                    SInt::new(&tokenstream_to_string(&expr[pos_start..pos_end]).unwrap()).unwrap(),
                ));

                if pos_start != pos_end {
                    expr.drain(pos_start..pos_end - 1);
                }

                expr[pos_start] = Token::Constant(sym).into();
            }

            // Take out any variables and make them point to the symbol table
            // All characters in an expression must be variables, so we keep going until there aren't any left
            while expr.iter().any(|t| t.is_char()) {
                let pos_start = expr.iter().position(|t| t.is_char()).unwrap();
                let pos_end = expr
                    .iter()
                    .skip(pos_start)
                    .position(|t| !t.is_char())
                    .unwrap_or_else(|| expr.len());

                let sym = Token::Symbol(tokenstream_to_string(&expr[pos_start..pos_end]).unwrap());

                if pos_start != pos_end {
                    expr.drain(pos_start..pos_end - 1);
                }

                expr[pos_start] = sym.into();
            }

            let exp = Expression::new(expr.iter().map(|t| Token::try_from(t).unwrap()).collect());
            tokens.push(Token::Expression(exp).into());

            //-----------------------------------------------------------------------------------

            // Clear the current expression
            expr = Vec::new();

            continue;
        }

        if in_expr {
            // Ignore any spaces in the expression
            if t != TokenStreamToken::Char(' ') {
                expr.push(t);
            }
            continue;
        }

        if &t == Token::LBracket {
            in_expr = true;
            continue;
        }

        tokens.push(t);
    }

    Ok(tokens)
}
