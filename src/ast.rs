use super::token::{Token, Literal};
use super::errors::LoxError;

#[derive(Debug)]
enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

#[derive(Debug)]
struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

#[derive(Debug)]
pub struct GroupingExpr {
    expression: Box<Expr>
}

#[derive(Debug)]
pub struct LiteralExpr {
    value: Literal,
}

#[derive(Debug)]
pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

pub trait Visitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, LoxError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, LoxError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, LoxError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, LoxError>;
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &dyn Visitor<T>) -> Result<T, LoxError> {
        visitor.visit_unary_expr(self)
    }
}

