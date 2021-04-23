use crate::token::TokenStream;
use crate::token::TokenType::Semicolon;
use crate::{statements, vm};
use crate::{Error, SyntaxError};
use std::fs;
use std::path::PathBuf;

/// A high level representation of an ofin program
pub struct Program {
    source: PathBuf,
    statements: Vec<Box<dyn vm::Runnable>>,
}

impl Program {
    pub fn new<P: Into<PathBuf>>(source: P) -> Result<Self, Error> {
        let source = source.into();
        let source_string = fs::read_to_string(&source)?;

        // Parse the source into a tokenstream
        let tokens = TokenStream::lex(source_string.clone());
        if let Err(pos) = tokens {
            let line = source_string[..pos].lines().count();
            let ctx = source_string[pos..].lines().next().unwrap().to_string();
            return Err(SyntaxError {
                file: source,
                t: "token",
                v: "unexpected",
                line,
                ctx,
            }
            .into());
        }
        let tokens = tokens.unwrap();
        println!("{:#?}", tokens);

        // Ensure we have an ending colon
        if tokens.get(tokens.len() - 1) != &Semicolon {
            return Err(SyntaxError {
                file: source,
                t: "semicolon",
                v: "expected",
                line: source_string.lines().count(),
                ctx: String::from("<no semicolon>"),
            }
            .into());
        }

        // Parse the tokenstream into a set of statements
        let statements = statements::parse(tokens);
        if let Err(pos) = statements {
            return Err(SyntaxError {
                file: source,
                t: "statement",
                v: "unrecognized",
                line: 0,                        // TODO
                ctx: String::from("good luck"), // TODO
            }
            .into());
        }
        let statements = statements.unwrap();
        println!("{:#?}", statements);

        Ok(Program { source, statements })
    }

    pub fn run(self) -> Result<(), Error> {
        vm::run(self.statements)
    }
}
