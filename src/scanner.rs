use super::token_type::TokenType;
use super::errors::TokenError;

#[derive(Debug, Clone)]
pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<TokenType>,
    start: u64,
    current: u64,
    line: u64,
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<TokenType> {
        loop {
            if self.is_at_end() {
                break;
            }
            self.start = self.current;
            self.scan_token();
        }
        let eof_token = TokenType::Eof;
        self.tokens.push(eof_token);
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u64
    }

    fn scan_token(&mut self) -> Result<(), TokenError> {
        let c = self.advance() as char; 
        let token = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            _ => return Err(TokenError::InvalidToken(c)) 
        };

        self.add_token(token);
        Ok(())
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current as usize]
    }

    fn add_token(&mut self, token: TokenType) {

    }

}
