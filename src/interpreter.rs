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
    pub environment: Environment,
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
                    Err(LoxError::RuntimeError {
                        error_type: RuntimeErrorType::ExpectedNumberOperand,
                    })
                }
            }
            TokenType::Bang => Ok(Object::from(!self.is_truthy(right))),
            _ => Err(LoxError::RuntimeError {
                error_type: RuntimeErrorType::UnreachableCode,
            }),
        }
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

                Err(LoxError::RuntimeError {
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            TokenType::Slash => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left / right));
                    }
                }

                Err(LoxError::RuntimeError {
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

                Err(LoxError::RuntimeError {
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
                Err(LoxError::RuntimeError {
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

                Err(LoxError::RuntimeError {
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

                Err(LoxError::RuntimeError {
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

                Err(LoxError::RuntimeError {
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

                Err(LoxError::RuntimeError {
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            //Handle '!='
            TokenType::BangEqual => Ok(Object::from(left != right)),

            //Handle '=='
            TokenType::EqualEqual => Ok(Object::from(left == right)),

            _ => Err(LoxError::RuntimeError {
                error_type: RuntimeErrorType::UnreachableCode,
            }),
        }
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Object, LoxError> {
        self.environment.get(expr.name.dup())
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
        if let Ok(value) = self.evaluate(&stmt.expression) {
            println!("{}", value);
        }

        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxError> {
        let mut value = Object::Nil;

        if stmt.initializer.is_some() {
            value = self.evaluate(stmt.initializer.as_ref().unwrap())?;
        }

        // TODO
        self.environment.define(stmt.name.lexeme.clone(), value);

        Ok(())
    }
}

impl Interpreter {
    pub fn evaluate(&self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    pub fn is_truthy(&self, obj: Object) -> bool {
        !(obj == Object::Nil || obj == Object::False)
    }

    pub fn interpret(&self, statements: &[Stmt]) {
        for statement in statements {
            self.execute(statement);
            /*
            {
                Ok(obj) => {
                    println!("Final result: {}", obj);
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
            */
        }
    }

    pub fn execute(&self, stmt: &Stmt) {
        match stmt.accept(self) {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }
}
