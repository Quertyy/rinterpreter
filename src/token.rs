use super::token_type::TokenType;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Identifier(s) => write!(f, "{}", s),
            Literal::Str(s) => write!(f, "\"{}\"", s),
            Literal::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub litteral: Option<Literal>,
    pub line: u64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, litteral: Option<Literal>, line: u64) -> Self {
        Self {
            token_type,
            lexeme,
            litteral,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}
