use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub litteral: Option<String>,
    pub line: u64,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, litteral: Option<String>, line: u64) -> Self {
        Self {
            token_type,
            lexeme,
            litteral,
            line,
        }
    }
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.litteral)
    }
}
