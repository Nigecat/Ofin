use crate::token::TokenStream;
use crate::{Error, SyntaxError};
use std::fs;
use std::path::PathBuf;

/// A high level representation of an ofin program
pub struct Program {
    source: PathBuf,
}

impl Program {
    pub fn new<P: Into<PathBuf>>(source: P) -> Result<Self, Error> {
        let source = source.into();
        let source_string = fs::read_to_string(&source)?;
        let tokens = TokenStream::lex(source_string.clone());
        if let Err(pos) = tokens {
            let line = source_string[..pos].lines().count();
            let ctx = source_string[pos..].lines().next().unwrap().to_string();
            return Err(SyntaxError {
                file: source,
                t: "token",
                line,
                ctx,
            }
            .into());
        }
        let tokens = tokens.unwrap();
        println!("{:#?}", tokens);

        Ok(Program { source })
    }

    pub fn run(&mut self) -> Result<(), Error> {
        // unimplemented!();
        Ok(())
    }
}
