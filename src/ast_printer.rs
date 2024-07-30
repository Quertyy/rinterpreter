use super::ast::*;
use super::errors::LoxError;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> Result<String, LoxError> {
        expr.accept(self)
    }

    pub fn parenthesize(&self, name: String, exprs: Vec<&Expr>) -> Result<String, LoxError> {
        let mut text = format!("({}", name);
        for expr in exprs {
            text.push(' ');
            text.push_str(&expr.accept(self).unwrap());
        }
        text.push(')');
        Ok(text)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, LoxError> {
        self.parenthesize(expr.operator.to_string(), vec![&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, LoxError> {
        self.parenthesize("group".to_string(), vec![&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, LoxError> {
        match &expr.value {
           Some(e) => Ok(e.to_string()),
           None => Ok("nil".to_string())
        }
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, LoxError> {
        self.parenthesize(expr.operator.lexeme.to_string(), vec![&expr.right])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::*;
    use crate::token_type::*;

    #[test]
    fn test_string() {
        let left = UnaryExpr {
            operator: Token::new(TokenType::Minus, '-'.to_string(), None, 1),
            right: Box::new(Expr::Literal(LiteralExpr { value: Some(Literal::Number(123.0)) }))
        };
        let operator = Token::new(TokenType::Star, '*'.to_string(), None, 1);
        let right = GroupingExpr {
            expression: Box::new(Expr::Literal(LiteralExpr { value: Some(Literal::Number(45.67)) }))
        };

        let expr = Expr::Binary(
            BinaryExpr { 
                left: Box::new(Expr::Unary(left)), 
                operator, 
                right: Box::new(Expr::Grouping(right)) 
            }
        );

        let ast = AstPrinter;
        let expected = "(* (- 123) (group 45.67))".to_string();
        assert_eq!(ast.print(expr).unwrap(), expected);
    }
}
