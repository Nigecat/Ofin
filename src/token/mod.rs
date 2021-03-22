mod rule;
use rule::Rule;

use crate::error::SyntaxError;
use crate::expression::Expression;
use std::fmt;
use std::ops::Deref;

#[deny(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum TokenType {
    Ident,
    Number,

    // ----- Keywords -----
    Import,
    Using,

    // ----- Characters -----
    Space,

    // ----- Special Characters -----
    Eol,
    Control,

    // ----- Blocks -----
    Target,
    Comment,
    Expression,
    Block,
}

macro_rules! rule {
    ($t: expr, $re: literal) => {
        Rule::new($t, $re, false)
    };
    ($t: expr, $re: literal, $firstlast: literal) => {
        Rule::new($t, $re, $firstlast)
    };
}

lazy_static! {
    static ref RULES: Vec<Rule> = vec![
        rule!(TokenType::Space, " "),
        rule!(TokenType::Eol, ";"),
        rule!(TokenType::Control, "[\r\n\t]"),
        rule!(TokenType::Ident, r#"\w+"#),
        rule!(TokenType::Number, r#"\d+"#),
        rule!(TokenType::Comment, "//.*?(?=(\r?\n|$))"),
        rule!(TokenType::Expression, r#"\(.*?\)"#, true),
        rule!(TokenType::Block, "{.*?}", true),
        rule!(TokenType::Target, "<.*?>", true),
        rule!(TokenType::Import, "import"),
        rule!(TokenType::Using, "using"),
    ];
}

pub struct Token {
    t: TokenType,
    literal: String,
    /// The expression bound to this token, this is guaranteed to be `Some` if `self.t == TokenType::Expression`
    expression: Option<Expression>,
}

impl Token {
    pub fn t(&self) -> TokenType {
        self.t
    }

    pub fn literal(self) -> String {
        self.literal
    }

    pub fn expression(self) -> Option<Expression> {
        self.expression
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({:?})", self.t, self.literal)
    }
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        self.t == *other
    }
}

impl Deref for Token {
    type Target = TokenType;

    fn deref(&self) -> &Self::Target {
        &self.t
    }
}

pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn lex(mut source: String) -> Result<Self, SyntaxError> {
        let mut tokens = Vec::new();
        'source: while !source.is_empty() {
            for rule in RULES.iter() {
                if let Some(index) = rule.find(&source) {
                    let mut literal: String = source.drain(0..index).collect();
                    if rule.firstlast() {
                        let mut chars = literal.chars();
                        chars.next();
                        chars.next_back();
                        literal = chars.collect();
                    }

                    let expression = if rule.t() == TokenType::Expression {
                        Some(Expression::parse(literal.clone()).ok_or_else(SyntaxError::new)?)
                    } else {
                        None
                    };

                    tokens.push(Token {
                        t: rule.t(),
                        literal,
                        expression,
                    });
                    continue 'source;
                }
            }
            // If no rule matched then there is a syntax error in the source
            return Err(SyntaxError::new());
        }

        Ok(TokenStream { tokens })
    }

    /// Filter out the supplied tokens from the tokestream
    pub fn filter(&mut self, tokens: &[TokenType]) {
        self.tokens.retain(|t| !tokens.contains(t));
    }

    pub fn tokens(self) -> Vec<Token> {
        self.tokens
    }
}

impl fmt::Debug for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut content = String::new();
        for token in self.tokens.iter() {
            if token.t() == TokenType::Eol {
                content.push('\n');
            } else {
                content.push_str(&format!("{:?} ", token));
            }
        }
        write!(f, "{}", content)
    }
}
