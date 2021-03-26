use super::{binding, Expression};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
/// An S-expression
pub enum S {
    Atom(char),
    Cons(char, (Option<Box<S>>, Option<Box<S>>)),
}

impl fmt::Debug for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                if let Some(l) = &rest.0 {
                    write!(f, " {}", l)?
                }
                if let Some(r) = &rest.1 {
                    write!(f, " {}", r)?
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Atom(char),
    Operator(char),
}

pub struct ExpressionLexer {
    tokens: Vec<Token>,
}

impl ExpressionLexer {
    /// Parse the given input to an expression
    pub fn lex<S: AsRef<str>>(input: S) -> Expression {
        let mut tokens = input
            .as_ref()
            .chars()
            .filter(|it| !it.is_ascii_whitespace())
            .map(|c| match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' => Token::Atom(c),
                _ => Token::Operator(c),
            })
            .collect::<Vec<_>>();
        tokens.reverse();

        let mut lexer = ExpressionLexer { tokens };
        let s = lexer.s(None);

        Expression { s }
    }

    /// Convert this expression into an S-expression
    fn s(&mut self, min_bp: Option<u8>) -> S {
        let min_bp = min_bp.unwrap_or(0);

        // Note that this can never return `None` (unless the input is empty)
        // Because of the check inside the loop
        let mut lhs = match self.next().unwrap() {
            Token::Atom(it) => S::Atom(it),
            Token::Operator('(') => {
                // If we have an opening ( parse until we reach a closing )
                let lhs = self.s(Some(0));
                // Double check the next character is the closing bracket
                assert_eq!(self.next(), Some(Token::Operator(')')));
                lhs
            }
            Token::Operator(op) => {
                let ((), r_bp) = binding::prefix(op);
                let rhs = self.s(Some(r_bp));
                S::Cons(op, (Some(Box::new(rhs)), None))
            }
        };

        loop {
            let op = match self.peek() {
                Some(Token::Operator(op)) => op,
                None => break, // Stop parsing when we run out of tokens
                t => panic!("bad token: {:?}", t),
            };

            if let Some((l_bp, r_bp)) = binding::infix(op) {
                if l_bp < min_bp {
                    break;
                }

                self.next();
                let rhs = self.s(Some(r_bp));
                lhs = S::Cons(op, (Some(Box::new(lhs)), Some(Box::new(rhs))));
                continue;
            }

            break;
        }

        lhs
    }

    /// Get the next token
    fn next(&mut self) -> Option<Token> {
        self.tokens.pop()
    }

    /// Peek the next token
    fn peek(&self) -> Option<Token> {
        self.tokens.last().copied()
    }
}
