use super::{binding, Expression};
use std::fmt;
use std::rc::Rc;

#[derive(Clone, PartialEq, Eq)]
/// An S-expression
pub enum S {
    Atom(char),
    Cons(char, [Rc<S>; 2]),
}

impl fmt::Display for S {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            S::Atom(i) => write!(f, "{}", i),
            S::Cons(head, rest) => {
                write!(f, "({}", head)?;
                for s in rest {
                    write!(f, " {}", s)?
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
        let mut lhs = match self.next() {
            Some(Token::Atom(it)) => S::Atom(it),
            t => panic!("bad token: {:?}", t),
        };

        loop {
            let op = match self.peek() {
                Some(Token::Operator(op)) => op,
                None => break, // Stop parsing when we run out of tokens
                t => panic!("bad token: {:?}", t),
            };
            let (l_bp, r_bp) = binding::infix(op);
            if l_bp < min_bp {
                break;
            }

            self.next();
            let rhs = self.s(Some(r_bp));
            lhs = S::Cons(op, [Rc::new(lhs), Rc::new(rhs)]);
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
