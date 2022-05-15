use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::errors::{LoxError, RuntimeErrorType};
use crate::expr::*;
use crate::stmt::*;
use crate::token::Object;
use crate::token_type::TokenType;

/**
 * This is the interpreter object.
 */
pub struct Interpreter {
    environment: RefCell<Rc<RefCell<Environment>>>,
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => {
                if let Object::Num(x) = right {
                    Ok(Object::Num(-x))
                } else {
                    Err(LoxError::Runtime {
                        error_type: RuntimeErrorType::ExpectedNumberOperand,
                    })
                }
            }
            TokenType::Bang => Ok(Object::from(!self.is_truthy(right))),
            _ => Err(LoxError::Runtime {
                error_type: RuntimeErrorType::UnreachableCode,
            }),
        }
    }

    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<Object, LoxError> {
        let value = self.evaluate(&expr.value)?;

        self.environment
            .borrow()
            .borrow_mut()
            .assign(expr.name.dup(), value.clone())?;

        Ok(value)
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left - right));
                    }
                }

                Err(LoxError::Runtime {
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            TokenType::Slash => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left / right));
                    }
                }

                Err(LoxError::Runtime {
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            // Handle number multiplication
            TokenType::Star => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left * right));
                    }
                }

                Err(LoxError::Runtime {
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            // Handle addition (number or string)
            TokenType::Plus => {
                // Handle 2 numbers
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left + right));
                    }
                }

                // Handle 2 strings
                if let Object::Str(left) = left {
                    if let Object::Str(right) = right {
                        let mut s = left;
                        s.push_str(&right);
                        return Ok(Object::from(s));
                    }
                }

                // TODO: Specific error for when 2 different type (a string and a number)
                Err(LoxError::Runtime {
                    error_type: RuntimeErrorType::ExpectedAddableOperands,
                })
            }

            // Comparison operators
            //Handle '>'
            TokenType::Greater => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left > right));
                    }
                }

                Err(LoxError::Runtime {
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            //Handle '>='
            TokenType::GreaterEqual => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left >= right));
                    }
                }

                Err(LoxError::Runtime {
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            //Handle '<'
            TokenType::Less => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left < right));
                    }
                }

                Err(LoxError::Runtime {
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            //Handle '<='
            TokenType::LessEqual => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left <= right));
                    }
                }

                Err(LoxError::Runtime {
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            //Handle '!='
            TokenType::BangEqual => Ok(Object::from(left != right)),

            //Handle '=='
            TokenType::EqualEqual => Ok(Object::from(left == right)),

            _ => Err(LoxError::Runtime {
                error_type: RuntimeErrorType::UnreachableCode,
            }),
        }
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Object, LoxError> {
        self.environment.borrow().borrow().get(expr.name.dup())
    }

    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.ttype == TokenType::Or {
            if self.is_truthy(left.clone()) {
                return Ok(left);
            }
        } else if !self.is_truthy(left.clone()) {
                return Ok(left);
        }

        self.evaluate(&expr.right)
    }

    fn visit_call_expr(&self, expr: &CallExpr) -> Result<Object, LoxError> {
        let callee = self.evaluate(&expr.callee)?;

        let arguments: Vec<Object> = Vec::new();
        for argument in expr.arguments {
            arguments.push(self.evaluate(&argument)?);
        }


        let callable = match callee {
            _ => ()
        };

        todo!()
        // callee.call(self, arguments)
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxError> {
        if let Err(e) = self.evaluate(&stmt.expression) {
            eprintln!("{}", e);
        }

        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);

        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxError> {
        let mut value = Object::Nil;

        if stmt.initializer.is_some() {
            value = self.evaluate(stmt.initializer.as_ref().unwrap())?;
        }

        self.environment
            .borrow()
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);

        Ok(())
    }

    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxError> {
        let env = Environment::from_enclosing(self.environment.borrow().clone());
        self.execute_block(&stmt.statements, env)
    }

    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<(), LoxError> {
        if self.is_truthy(self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.then_branch)?;
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<(), LoxError> {
        while self.is_truthy(self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.body)?;
        }

        Ok(())
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: RefCell::new(Rc::new(RefCell::new(Environment::new()))),
        }
    }

    pub fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    pub fn is_truthy(&self, obj: Object) -> bool {
        !(obj == Object::Nil || obj == Object::False)
    }

    pub fn interpret(&self, statements: &[Stmt]) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    pub fn execute(&self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)
    }

    pub fn execute_block(&self, stmts: &[Stmt], env: Environment) -> Result<(), LoxError> {
        let prev_env = self.environment.replace(Rc::new(RefCell::new(env)));

        let ret = stmts.iter().try_for_each(|stmt| self.execute(stmt));

        self.environment.replace(prev_env);

        ret
    }
}
