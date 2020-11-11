use super::{TokenStream, TokenStreamToken};
use crate::application::Application;
use crate::errors::TokenizerError;
use crate::tokens::Token;

/// Tokenize expressions
pub fn pass_two(
    tokenstream: TokenStream,
    mut application: &mut Application,
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

            println!("{:?}", expr);

            //-----------------------------------------------------------------------------------

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

    /*
    // while let Some(pos) = tokenstream.iter().position(|t| t == Token::LBracket) {
    let pos = tokenstream
        .iter()
        .position(|t| t == Token::LBracket)
        .unwrap();

    let mut expr: Vec<&TokenStreamToken> = Vec::new();

    while &mut tokenstream
        .iter()
        .nth(pos)
        .ok_or_else(|| TokenizerError::new("Missing closing bracket"))?
        != &mut Token::RBracket
    {
        expr.push(&tokenstream[pos]);
        tokenstream.remove(pos);
    }

    print!("{:?}", expr);
    // }
    */

    Ok(tokens)
}
