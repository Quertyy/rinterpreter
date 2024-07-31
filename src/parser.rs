use std::fmt::Binary;

use crate::token::Literal;
use crate::token_type::TokenType;
use crate::{BinaryExpr, GroupingExpr, LiteralExpr, UnaryExpr};

use super::token::Token;
use super::ast::Expr;

#[derive(Debug)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: u32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        loop {
            if !self.match_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
                break;
            }

            let operator = self.previous();
            let right = self.comparison();
            let binary_expr = BinaryExpr::new(expr, operator.clone(), right);
            expr = Expr::Binary(binary_expr);
        }
        expr
    }

    fn match_types(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous();
    }
 
    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current as usize).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get((self.current - 1) as usize).unwrap()
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        loop {
            if !self.match_types(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
                break;
            }
            let operator = self.previous();
            let right = self.term();
            let binary_expr = BinaryExpr::new(expr, operator.clone(), right);
            expr = Expr::Binary(binary_expr);
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        loop {
            if !self.match_types(vec![TokenType::Minus, TokenType::Plus]) {
                break;
            }
            let operator = self.previous();
            let right = self.factor();
            let binary_expr = BinaryExpr::new(expr, operator.clone(), right);
            expr = Expr::Binary(binary_expr)
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        loop {
            if !self.match_types(vec![TokenType::Slash, TokenType::Start]) {
                break;
            }
            let operator = self.previous();
            let right = self.unary();
            let binary_expr = BinaryExpr::new(expr, operator.clone, right);
            expr = Expr::Binary(binary_expr)
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_types(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();
            let unary_expr = UnaryExpr::new(operator.clone(), right);
            return Expr::Unary(unary_expr);
        }
        self.primary().unwrap()
    }

    fn primary(&mut self) -> Option<Expr> {
        if self.match_types(vec![TokenType::False]) {
            return Some(Expr::Literal(LiteralExpr::new(Some(Literal::False))));
        }
        if self.match_types(vec![TokenType::True]) {
            return Some(Expr::Literal(LiteralExpr::new(Some(Literal::True))));
        }
        if self.match_types(vec![TokenType::Nil]) {
            return Some(Expr::Literal(LiteralExpr::new(None)));
        }

        if self.match_types(vec![TokenType::Number, TokenType::String]) {
            return Some(Expr::Literal(LiteralExpr::new(self.previous().litteral)));
        }

        if self.match_types(vec![TokenType::LeftParen]) {
            let expr = self.expression();
            let text = "Expect ')' after expression".to_string();
            self.consume(TokenType::RightParen, text); 
            let grouping_expr = GroupingExpr::new(expr);
            return Some(Expr::Grouping(grouping_expr));
        }

        None
    }
}


