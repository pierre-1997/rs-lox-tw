use std::cell::RefCell;
use std::collections::HashMap;

use crate::errors::{LoxResult, ResolverErrorType};
use crate::expr::*;
use crate::interpreter::Interpreter;
use crate::object::Object;
use crate::stmt::*;
use crate::token::Token;

#[derive(PartialEq, Clone, Copy)]
enum FunctionType {
    Void,
    Function,
    Method,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: RefCell<Vec<HashMap<String, bool>>>,
    current_function: FunctionType,
}

impl<'a> StmtVisitor<()> for Resolver<'a> {
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> Result<(), LoxResult> {
        self.begin_scope();
        self.resolve_stmts(statements)?;
        self.end_scope();

        Ok(())
    }
    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), LoxResult> {
        self.resolve_expr(expression)?;
        Ok(())
    }

    fn visit_function_stmt(
        &mut self,
        name: &Token,
        params: &[Token],
        body: &[Stmt],
    ) -> Result<(), LoxResult> {
        self.declare(name)?;
        self.define(name);

        self.resolve_function(params, body, FunctionType::Function)?;
        Ok(())
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Stmt>,
    ) -> Result<(), LoxResult> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(then_branch)?;
        if let Some(else_branch) = else_branch {
            self.resolve_stmt(else_branch)?;
        }

        Ok(())
    }

    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<(), LoxResult> {
        self.resolve_expr(expression)?;
        Ok(())
    }

    fn visit_return_stmt(
        &mut self,
        keyword: &Token,
        value: &Option<Expr>,
    ) -> Result<(), LoxResult> {
        if self.current_function == FunctionType::Void {
            return Err(LoxResult::Resolver {
                token: keyword.to_owned(),
                error_type: ResolverErrorType::TopLevelReturn,
            });
        }
        if let Some(value) = value {
            self.resolve_expr(value)?;
        }

        Ok(())
    }

    fn visit_var_stmt(
        &mut self,
        name: &Token,
        initializer: &Option<Expr>,
    ) -> Result<(), LoxResult> {
        self.declare(name)?;
        if initializer.is_some() {
            self.resolve_expr(initializer.as_ref().unwrap())?;
        }
        self.define(name);

        Ok(())
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), LoxResult> {
        self.resolve_expr(condition)?;
        self.resolve_stmt(body)?;

        Ok(())
    }

    /**
     * Function used to resolve class methods.
     */
    fn visit_class_stmt(&mut self, name: &Token, methods: &[Stmt]) -> Result<(), LoxResult> {
        // Declare and define the class name
        self.declare(name)?;
        self.define(name);

        // Start the class scope
        self.begin_scope();
        // Insert the 'this' keyword as it should always be defined
        self.scopes
            .borrow_mut()
            .last_mut()
            .unwrap()
            .insert("this".to_string(), true);

        // For each method of the class, resolve it
        for method in methods {
            // Each statement in the 'methods' argument should be of the underlying
            // variant `Stmt::Function`.
            if let Stmt::Function {
                name: _,
                params,
                body,
            } = method
            {
                self.resolve_function(params, body, FunctionType::Method)?;
            } else {
                unreachable!()
            };
        }

        // End the class scope
        self.end_scope();

        Ok(())
    }
}

impl<'a> ExprVisitor<()> for Resolver<'a> {
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<(), LoxResult> {
        self.resolve_expr(value)?;
        self.resolve_local(name);
        Ok(())
    }

    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        _operator: &Token,
        right: &Expr,
    ) -> Result<(), LoxResult> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;

        Ok(())
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        _paren: &Token,
        arguments: &[Expr],
    ) -> Result<(), LoxResult> {
        self.resolve_expr(callee)?;
        self.resolve_exprs(arguments)?;

        Ok(())
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        _operator: &Token,
        right: &Expr,
    ) -> Result<(), LoxResult> {
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;

        Ok(())
    }

    fn visit_unary_expr(&mut self, _operator: &Token, right: &Expr) -> Result<(), LoxResult> {
        self.resolve_expr(right)?;

        Ok(())
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<(), LoxResult> {
        self.resolve_expr(expression)?;
        Ok(())
    }

    fn visit_literal_expr(&mut self, _value: &Option<Object>) -> Result<(), LoxResult> {
        Ok(())
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<(), LoxResult> {
        if !self.scopes.borrow().is_empty()
            && self.scopes.borrow().last().unwrap().get(&name.lexeme) == Some(&false)
        {
            return Err(LoxResult::Resolver {
                token: name.clone(),
                error_type: ResolverErrorType::VariableNotInitialized,
            });
        }

        self.resolve_local(name);

        Ok(())
    }

    fn visit_get_expr(&mut self, object: &Expr, _name: &Token) -> Result<(), LoxResult> {
        self.resolve_expr(object)?;
        Ok(())
    }

    fn visit_set_expr(
        &mut self,
        object: &Expr,
        _name: &Token,
        value: &Expr,
    ) -> Result<(), LoxResult> {
        self.resolve_expr(value)?;
        self.resolve_expr(object)?;

        Ok(())
    }

    fn visit_this_expr(&mut self, keyword: &Token) -> Result<(), LoxResult> {
        self.resolve_local(keyword);
        Ok(())
    }
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: RefCell::new(Vec::new()),
            current_function: FunctionType::Void,
        }
    }

    /**
     * Function called when entering a new scope. It simply appends a new HashMap into the
     * `self.scopes` property of the `Resolver`.
     */
    fn begin_scope(&self) {
        self.scopes.borrow_mut().push(HashMap::new());
    }

    /**
     * Function called when leaving a scope. It simply pops the lastly appended scope from the
     * `self.scopes` property of the `Resolver`.
     */
    fn end_scope(&self) {
        self.scopes.borrow_mut().pop();
    }

    fn declare(&self, name: &Token) -> Result<(), LoxResult> {
        if self.scopes.borrow().is_empty() {
            return Ok(());
        }

        if self
            .scopes
            .borrow_mut()
            .last_mut()
            .unwrap()
            .contains_key(&name.lexeme)
        {
            return Err(LoxResult::Resolver {
                token: name.clone(),
                error_type: ResolverErrorType::VariableAlreadyExists,
            });
        }

        self.scopes
            .borrow_mut()
            .last_mut()
            .unwrap()
            .insert(name.lexeme.clone(), false);

        Ok(())
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

    pub fn resolve_stmts(&mut self, stmts: &[Stmt]) -> Result<(), LoxResult> {
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

    /**
     * Calls the interpreter's resolve function once the object is found.
     */
    fn resolve_local(&mut self, name: &Token) {
        for i in self.scopes.borrow().len()..0 {
            if self.scopes.borrow()[i].contains_key(&name.lexeme) {
                self.interpreter
                    .resolve(name, self.scopes.borrow().len() - 1 - i);
                break;
            }
        }
    }

    /**
     * Function called to declare and define a new function to the resolver.
     *
     * Note: It modifies and sets back the function type that is currently being resolved.
     */
    fn resolve_function(
        &mut self,
        params: &[Token],
        body: &[Stmt],
        function_type: FunctionType,
    ) -> Result<(), LoxResult> {
        // Store the surrounding function type
        let ftype = self.current_function;
        // Set the current function type to the one we're currently declaring
        self.current_function = function_type;

        // Start a new scope
        self.begin_scope();

        // Declare and define each parameter of the function into the current scope
        for param in params {
            self.declare(param)?;
            self.define(param);
        }
        // Resolve the list of statements that compose the body of the function
        self.resolve_stmts(body)?;

        // End the function's scope
        self.end_scope();

        // Set back the current function type being resolve to that we were before on
        self.current_function = ftype;

        Ok(())
    }
}
