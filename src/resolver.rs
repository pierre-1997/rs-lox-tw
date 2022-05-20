use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::errors::{LoxResult, ResolverErrorType};
use crate::expr::*;
use crate::interpreter::Interpreter;
use crate::stmt::*;
use crate::token::Token;

pub struct Resolver<'a> {
    interpreter: &'a Interpreter,
    scopes: RefCell<Vec<HashMap<String, bool>>>,
}

impl<'a> StmtVisitor<()> for Resolver<'a> {
    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxResult> {
        self.begin_scope();
        self.resolve_stmts(&stmt.statements)?;
        self.end_scope();

        Ok(())
    }

    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxResult> {
        todo!()
    }
    fn visit_function_stmt(&self, stmt: &FunctionStmt) -> Result<(), LoxResult> {
        todo!()
    }
    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<(), LoxResult> {
        todo!()
    }
    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxResult> {
        todo!()
    }
    fn visit_return_stmt(&self, stmt: &ReturnStmt) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxResult> {
        self.declare(&stmt.name);
        if stmt.initializer.is_some() {
            self.resolve_expr(stmt.initializer.as_ref().unwrap())?;
        }
        self.define(&stmt.name);

        Ok(())
    }

    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<(), LoxResult> {
        todo!()
    }
}

impl<'a> ExprVisitor<()> for Resolver<'a> {
    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<(), LoxResult> {
        todo!()
    }
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<(), LoxResult> {
        todo!()
    }
    fn visit_call_expr(&self, expr: &CallExpr) -> Result<(), LoxResult> {
        todo!()
    }
    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<(), LoxResult> {
        todo!()
    }
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<(), LoxResult> {
        todo!()
    }
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<(), LoxResult> {
        todo!()
    }
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<(), LoxResult> {
        if !self.scopes.borrow().is_empty()
            && self
                .scopes
                .borrow()
                .last()
                .unwrap()
                .get(&expr.name.lexeme)
                .is_some()
        {
            return Err(LoxResult::Resolver {
                token: expr.name.dup(),
                error_type: ResolverErrorType::VariableNotInitialized,
            });
        }

        // self.resolve_local(&Expr::Variable(), &expr.name);

        Ok(())
    }
}

impl<'a> Resolver<'a> {
    fn begin_scope(&self) {
        self.scopes.borrow_mut().push(HashMap::new());
    }

    fn end_scope(&self) {
        self.scopes.borrow_mut().pop();
    }

    fn declare(&self, name: &Token) {
        if self.scopes.borrow().is_empty() {
            return;
        }

        self.scopes
            .borrow_mut()
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), false);
    }

    fn define(&self, name: &Token) {
        if self.scopes.borrow().is_empty() {
            return;
        }

        self.scopes
            .borrow_mut()
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), true);
    }

    fn resolve_stmts(&self, stmts: &Vec<Rc<Stmt>>) -> Result<(), LoxResult> {
        stmts.iter().try_for_each(|stmt| self.resolve_stmt(stmt))
    }

    fn resolve_stmt(&self, stmt: &Stmt) -> Result<(), LoxResult> {
        stmt.accept(self)
    }

    fn resolve_exprs(&self, exprs: &[Expr]) -> Result<(), LoxResult> {
        exprs.iter().try_for_each(|expr| self.resolve_expr(expr))
    }

    fn resolve_expr(&self, expr: &Expr) -> Result<(), LoxResult> {
        expr.accept(self)
    }

    fn resolve_local(&self, expr: &Expr, name: &Token) {
        for i in self.scopes.borrow().len()..0 {
            if self.scopes.borrow()[i].contains_key(&name.lexeme) {
                // self.interpreter
                // .resolve(expr, self.scopes.borrow().len() - 1 - i);
                break;
            }
        }
    }
}
