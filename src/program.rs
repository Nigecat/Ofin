use crate::token::TokenStream;
use crate::OfinError;
use std::fs;
use std::path::PathBuf;

/// An ofin program
pub struct Program {}

impl Program {
    /// Parse an ofin program from a file
    pub fn parse_from_file(file: PathBuf) -> Result<Self, OfinError> {
        let source = fs::read_to_string(&file).map_err(|_| OfinError::FileNotFound(file))?;
        let tokens = TokenStream::lex(source)?;
        debug!("{:#?}", tokens);

        unimplemented!();
    }

    /// Parse an ofin program from a string
    ///
    /// NOTE: `import` statements will cause a runtime error (`using` statements are fine)
    pub fn parse_from_str(source: String) -> Result<Self, OfinError> {
        unimplemented!();
    }

    /// Run this program
    pub fn run(&mut self) -> Result<Self, OfinError> {
        unimplemented!();
    }
}
