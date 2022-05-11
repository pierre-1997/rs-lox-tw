use crate::errors::ExprError;
use crate::expr::*;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> Result<String, ExprError> {
        expr.accept(self)
    }

    pub fn parenthesize(&self, name: &str, exprs: &[&Expr]) -> Result<String, ExprError> {
        let mut ret = String::new();

        // Open parenthesis + name
        ret.push('(');
        ret.push_str(name);

        // For each child expr, print it here
        for expr in exprs {
            ret.push(' ');
            ret.push_str(&expr.accept(self)?);
        }

        // Closing parenthesis
        ret.push(')');

        Ok(ret)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<String, ExprError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<String, ExprError> {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<String, ExprError> {
        self.parenthesize("group", &[&expr.expression])
    }

    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<String, ExprError> {
        if let Some(v) = &expr.value {
            Ok(v.to_string())
        } else {
            Ok("nil".to_string())
        }
    }
}
