use crate::token::Token;
use super::token_type::TokenType;
use super::errors::TokenError;

#[derive(Debug, Clone)]
pub struct Scanner {
    source: Vec<u8>,
    tokens: Vec<Token>,
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

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        loop {
            if self.is_at_end() {
                break;
            }
            self.start = self.current;
            self.scan_token();
        }
        let eof_token = Token::new(TokenType::Eof, "".to_string(), None, self.line);
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
            '!' => {
                if self.match_next_lexeme('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            },
            '=' => {
                if self.match_next_lexeme('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            },
            '<' => {
                if self.match_next_lexeme('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            },
            '>' => {
                if self.match_next_lexeme('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            },
            '/' => {
                if self.match_next_lexeme('/') {
                    loop {
                        if self.peek() != '\n' && self.is_at_end() {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    return Ok(())
                } else {
                    TokenType::Slash
                }
            },
            _ => return Err(TokenError::InvalidToken(self.line, c)) 
        };

        self.add_token(token);
        Ok(())
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current as usize]
    }

    fn add_token(&mut self, token: TokenType) {
        self.add_token_with_literal(token, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let start = self.start as usize;
        let current = self.current as usize;
        let text = String::from_utf8(self.source[start..current].to_vec()).expect("Invalid UTF-8");
        let new_token = Token::new(token_type, text, literal, self.line);
        self.tokens.push(new_token);
    }

    fn match_next_lexeme(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current as usize] as char != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0'
        }
        return self.source[self.current as usize] as char
    }
}
