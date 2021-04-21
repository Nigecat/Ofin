use crate::token::TokenStream;
use crate::Error;
use std::fs;
use std::path::PathBuf;

/// A high level representation of an ofin program
pub struct Program {
    source: PathBuf,
}

impl Program {
    pub fn new<P: Into<PathBuf>>(source: P) -> Result<Self, Error> {
        let source = source.into();
        let tokens = TokenStream::lex(fs::read_to_string(&source)?);
        println!("{:#?}", tokens);

        Ok(Program { source })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        // unimplemented!();
        Ok(())
    }
}
