use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub litteral: Option<String>,
    pub line: u64,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, litteral: Option<String>, line: u64) -> Self {
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
        write!(f, "{:#?} {} {:?}", self.token_type, self.lexeme, self.litteral)
    }
}
