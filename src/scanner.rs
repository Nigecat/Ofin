use crate::token::{Token, TokenType};
use crate::OfinError;
use std::iter;

pub struct Scanner;

impl Scanner {
    pub fn scan<I: Iterator<Item = char> + iter::ExactSizeIterator>(source: I) -> Vec<Token> {
        let mut source = source.peekable();
        let tokens = Vec::new();

        let mut row = 0;
        let mut column = 0;
        let mut position = 0;

        while position <= source.len() {
            let c = source.nth(position).unwrap();
        }

        tokens
    }
}
