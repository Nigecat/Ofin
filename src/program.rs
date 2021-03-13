use crate::token::TokenStream;
use crate::token::TokenType;
use crate::Error;
use std::fs;
use std::path::PathBuf;

/// An ofin program
#[derive(Debug)]
pub struct Program {}

impl Program {
    /// Parse an ofin program from a file
    pub fn parse_from_file(file: PathBuf) -> Result<Self, Error> {
        let source = fs::read_to_string(&file).map_err(|_| Error::FileNotFound(file))?;
        let mut tokens = TokenStream::lex(source)?;

        // Remove any control characters and spaces from the tokens
        tokens.filter(&[TokenType::Control, TokenType::Space]);

        // Resolve any file imports
        while let Some(pos) = tokens.iter().position(|t| *t == TokenType::Import) {
            trace!("{}", pos);
        }

        Ok(Program {})
    }

    /// Parse an ofin program from a string
    ///
    /// NOTE: `import` statements will cause a runtime error (`using` statements are fine)
    pub fn parse_from_str(_source: String) -> Result<Self, Error> {
        unimplemented!();
    }

    /// Run this program
    pub fn run(&mut self) -> Result<Self, Error> {
        unimplemented!();
    }
}
