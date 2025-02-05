use crate::token::{Token, Literal};
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
            if let Err(e) = self.scan_token() {
                println!("{e}");
            }
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
                } else if self.match_next_lexeme('*') {
                    return self.block_comment();
                } else {
                    TokenType::Slash
                }
            },
            ' ' | '\r' | '\t' => return Ok(()),
            '\n' => {
                self.line += 1;
                return Ok(())
            },
            '"' => {
                return self.string()
            },
            'o' => {
                if self.peek() == 'r' {
                    TokenType::Or
                } else {
                    return Ok(());
                }
            }
            _ => {
                if c.is_ascii_digit() {
                    self.number();
                    return Ok(());
                } else if self.is_alpha(c) {
                    self.identifier();
                    return Ok(());
                } else {
                    return Err(TokenError::InvalidToken(self.line, c));
                }
            },
        };

        self.add_token(token);
        Ok(())
    }

    fn advance(&mut self) -> u8 {
        self.current += 1;
        self.source[(self.current - 1) as usize]
    }

    fn add_token(&mut self, token: TokenType) {
        self.add_token_with_literal(token, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
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
        self.source[self.current as usize] as char
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= (self.source.len() as u64) {
            return '\0'
        }
        self.source[(self.current + 1) as usize] as char
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_ascii_digit()
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn string(&mut self) -> Result<(), TokenError> {
        loop {
            if self.peek() == '"' || self.is_at_end() {
                break;
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(TokenError::UnterminatedString(self.line));
        }
        self.advance();
        let start = (self.start + 1) as usize;
        let current = (self.current - 1) as usize;
        let value = String::from_utf8(self.source[start..current].to_vec()).expect("Invalid UTF-8");
        self.add_token_with_literal(TokenType::String, Some(Literal::Str(value)));
        Ok(())
    }

    fn block_comment(&mut self) -> Result<(), TokenError> {
        loop {
            if self.peek() == '*' && self.peek_next() == '/' || self.is_at_end() {
                break;
            }
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(TokenError::UnterminatedBlockComment(self.line))
        }
        Ok(())
    }

    fn number(&mut self) {
        loop {
            if !self.peek().is_ascii_digit() {
                break;
            }
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            loop {
                if !self.peek().is_ascii_digit() {
                    break;
                }
                self.advance();
            }
        }
        let number: f64 = String::from_utf8(self.source[self.start as usize..self.current as usize].to_vec()).expect("Invalid UTF-8").parse().unwrap();
        self.add_token_with_literal(TokenType::Number, Some(Literal::Number(number)));
    }

    fn identifier(&mut self) {
        loop {
            if !self.is_alpha_numeric(self.peek()) {
                break;
            } 
            self.advance();
        }
        let text = String::from_utf8(self.source[self.start as usize..self.current as usize].to_vec()).expect("Invalid UTF-8");
        let token = match serde_json::from_str::<TokenType>(text.as_str()) {
            Ok(token_type) => token_type,
            Err(_) => TokenType::Identifier
        };
        self.add_token(token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn construct_scanner(source: &str) -> Scanner {
        let b = source.as_bytes().to_vec();
        Scanner::new(b)
    }

    fn assert_tokens(token: Token, token_type: TokenType) {
        assert_eq!(token.token_type, token_type);
    }

    #[test]
    fn test_and() {
        let mut scanner = construct_scanner("and");
        let tokens = scanner.scan_tokens();
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
        assert_eq!(tokens[0].lexeme, "and");
    }

    #[test]
    fn test_left_paren() {
        let mut scanner = construct_scanner("(");
        let tokens = scanner.scan_tokens();
        assert_tokens(tokens[0].clone(), TokenType::LeftParen);
    }

    #[test]
    fn test_right_paren() {
        let mut scanner = construct_scanner(")");
        let tokens = scanner.scan_tokens();
        assert_tokens(tokens[0].clone(), TokenType::RightParen);
    }

    #[test]
    fn test_left_brace() {
        let mut scanner = construct_scanner("{");
        let tokens = scanner.scan_tokens();
        assert_tokens(tokens[0].clone(), TokenType::LeftBrace);
    }

    #[test]
    fn test_right_brace() {
        let mut scanner = construct_scanner("}");
        let tokens = scanner.scan_tokens();
        assert_tokens(tokens[0].clone(), TokenType::RightBrace);
    }

    #[test]
    fn test_comma() {
        let mut scanner = construct_scanner(",");
        let tokens = scanner.scan_tokens();
        assert_tokens(tokens[0].clone(), TokenType::Comma);
    }

    #[test]
    fn test_dot() {
        let mut scanner = construct_scanner(".");
        let tokens = scanner.scan_tokens();
        assert_tokens(tokens[0].clone(), TokenType::Dot);
    }
}
