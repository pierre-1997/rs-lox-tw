use std::cell::RefCell;
use std::collections::HashMap;

use crate::errors::{LoxResult, ResolverErrorType};
use crate::expr::*;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::stmt::*;
use crate::token::Token;

pub struct Resolver<'a> {
    interpreter: &'a Interpreter,
    scopes: RefCell<Vec<HashMap<String, bool>>>,
}

impl<'a> StmtVisitor<()> for Resolver<'a> {
    fn visit_block_stmt(&mut self, statements: &Vec<Stmt>) -> Result<(), LoxResult> {
        self.begin_scope();
        self.resolve_stmts(statements)?;
        self.end_scope();

        Ok(())
    }
    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_function_stmt(
        &mut self,
        name: &Token,
        params: &Vec<Token>,
        body: &Vec<Stmt>,
    ) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Stmt>,
    ) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_return_stmt(
        &mut self,
        keyword: &Token,
        value: &Option<Expr>,
    ) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_var_stmt(
        &mut self,
        name: &Token,
        initializer: &Option<Expr>,
    ) -> Result<(), LoxResult> {
        self.declare(name);
        if initializer.is_some() {
            self.resolve_expr(initializer.as_ref().unwrap())?;
        }
        self.define(name);

        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), LoxResult> {
        todo!()
    }
}

impl<'a> ExprVisitor<()> for Resolver<'a> {
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &Vec<Expr>,
    ) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_literal_expr(&mut self, value: &Option<Object>) -> Result<(), LoxResult> {
        todo!()
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<(), LoxResult> {
        if !self.scopes.borrow().is_empty()
            && self
                .scopes
                .borrow()
                .last()
                .unwrap()
                .get(&name.lexeme)
                .is_some()
        {
            return Err(LoxResult::Resolver {
                token: name.clone(),
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

    fn resolve_stmts(&mut self, stmts: &Vec<Stmt>) -> Result<(), LoxResult> {
        stmts.iter().try_for_each(|stmt| self.resolve_stmt(stmt))
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), LoxResult> {
        stmt.accept(self)
    }

    fn resolve_exprs(&mut self, exprs: &[Expr]) -> Result<(), LoxResult> {
        exprs.iter().try_for_each(|expr| self.resolve_expr(expr))
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), LoxResult> {
        expr.accept(self)
    }

    fn resolve_local(&self, expr: &Expr, name: &Token) {
        for i in self.scopes.borrow().len()..0 {
            if self.scopes.borrow()[i].contains_key(&name.lexeme) {
                // TODO: HERE
                // self.interpreter.resolve(name, self.scopes.borrow().len() - 1 - i);
                break;
            }
        }
    }
}
