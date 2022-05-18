use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::Environment;
use crate::errors::{LoxResult, RuntimeErrorType};
use crate::expr::*;
use crate::lox_callable::LoxCallable;
use crate::lox_function::LoxFunction;
use crate::lox_native::NativeFunction;
use crate::native_functions::NativeClock;
use crate::object::Object;
use crate::stmt::*;
use crate::token::Token;
use crate::token_type::TokenType;

/**
 * This is the interpreter object.
 */
pub struct Interpreter {
    environment: RefCell<Rc<RefCell<Environment>>>,
    pub env_globals: Rc<RefCell<Environment>>,
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<Object, LoxResult> {
        Ok(expr.value.clone().unwrap())
    }

    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<Object, LoxResult> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => {
                if let Object::Num(x) = right {
                    Ok(Object::Num(-x))
                } else {
                    Err(LoxResult::Runtime {
                        error_type: RuntimeErrorType::ExpectedNumberOperand,
                    })
                }
            }
            TokenType::Bang => Ok(Object::from(!self.is_truthy(right))),
            _ => Err(LoxResult::Runtime {
                error_type: RuntimeErrorType::UnreachableCode,
            }),
        }
    }

    fn visit_assign_expr(&self, expr: &AssignExpr) -> Result<Object, LoxResult> {
        let value = self.evaluate(&expr.value)?;

        self.environment
            .borrow()
            .borrow_mut()
            .assign(expr.name.dup(), value.clone())?;

        Ok(value)
    }

    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<Object, LoxResult> {
        self.evaluate(&expr.expression)
    }

    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<Object, LoxResult> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left - right));
                    }
                }

                Err(LoxResult::Runtime {
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            TokenType::Slash => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left / right));
                    }
                }

                Err(LoxResult::Runtime {
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

                Err(LoxResult::Runtime {
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
                Err(LoxResult::Runtime {
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

                Err(LoxResult::Runtime {
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

                Err(LoxResult::Runtime {
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

                Err(LoxResult::Runtime {
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

                Err(LoxResult::Runtime {
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            //Handle '!='
            TokenType::BangEqual => Ok(Object::from(left != right)),

            //Handle '=='
            TokenType::EqualEqual => Ok(Object::from(left == right)),

            _ => Err(LoxResult::Runtime {
                error_type: RuntimeErrorType::UnreachableCode,
            }),
        }
    }

    fn visit_variable_expr(&self, expr: &VariableExpr) -> Result<Object, LoxResult> {
        self.look_up_env(&expr.name)
    }

    fn visit_logical_expr(&self, expr: &LogicalExpr) -> Result<Object, LoxResult> {
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

    fn visit_call_expr(&self, expr: &CallExpr) -> Result<Object, LoxResult> {
        // Get the expression's callee
        let callee = self.evaluate(&expr.callee)?;

        // Optional vector of arguments
        let mut arguments: Vec<Object> = Vec::new();
        // Evaluate each calling argument
        for argument in &expr.arguments {
            arguments.push(self.evaluate(argument)?);
        }

        // Try to interpret the callee as a callable object (e.g function or class)
        let called_function: Rc<dyn LoxCallable> = match callee {
            // Check for native function
            Object::Native(native) => native.function.clone(),
            // Check for defined function
            Object::Function(function) => function,
            // Otherwise, this is not a callable object type, return an error.
            _ => {
                return Err(LoxResult::Runtime {
                    error_type: RuntimeErrorType::InvalidCallObjectType,
                });
            }
        };

        // Check called function's arity and return error if incorrect
        if arguments.len() != called_function.arity() {
            return Err(LoxResult::Runtime {
                error_type: RuntimeErrorType::InvalidArgsCount,
            });
        }

        // Return the function's call result
        called_function.call(self, arguments)
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&self, stmt: &ExpressionStmt) -> Result<(), LoxResult> {
        if let Err(e) = self.evaluate(&stmt.expression) {
            eprintln!("{}", e);
        }

        Ok(())
    }

    fn visit_print_stmt(&self, stmt: &PrintStmt) -> Result<(), LoxResult> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);

        Ok(())
    }

    fn visit_var_stmt(&self, stmt: &VarStmt) -> Result<(), LoxResult> {
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

    fn visit_block_stmt(&self, stmt: &BlockStmt) -> Result<(), LoxResult> {
        let env = Environment::from_enclosing(self.environment.borrow().clone());
        self.execute_block(&stmt.statements, env)
    }

    fn visit_if_stmt(&self, stmt: &IfStmt) -> Result<(), LoxResult> {
        if self.is_truthy(self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.then_branch)?;
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    /**
     * This function executes a return statement.
     *
     * Note: This function will use the LoxResult enum in order to return an actual value instead
     * of an error.
     */
    fn visit_return_stmt(&self, stmt: &ReturnStmt) -> Result<(), LoxResult> {
        if let Some(v) = &stmt.value {
            Err(LoxResult::ReturnValue {
                value: self.evaluate(v)?,
            })
        } else {
            Err(LoxResult::ReturnValue { value: Object::Nil })
        }
    }

    fn visit_while_stmt(&self, stmt: &WhileStmt) -> Result<(), LoxResult> {
        while self.is_truthy(self.evaluate(&stmt.condition)?) {
            self.execute(&stmt.body)?;
        }

        Ok(())
    }

    fn visit_function_stmt(&self, stmt: &FunctionStmt) -> Result<(), LoxResult> {
        // Instanciate a new function object using its statement
        let function = Object::Function(Rc::new(LoxFunction::new(stmt)));

        // Define the function in the current environment
        self.environment
            .borrow_mut()
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), function);

        Ok(())
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));

        globals.borrow_mut().define(
            "clock".to_string(),
            Object::Native(Rc::new(NativeFunction {
                function: Rc::new(NativeClock {}),
            })),
        );

        Interpreter {
            environment: RefCell::new(Rc::clone(&globals)),
            env_globals: Rc::clone(&globals),
        }
    }

    pub fn evaluate(&self, expr: &Expr) -> Result<Object, LoxResult> {
        expr.accept(self)
    }

    pub fn is_truthy(&self, obj: Object) -> bool {
        !(obj == Object::Nil || obj == Object::False)
    }

    pub fn interpret(&self, statements: &[Stmt]) -> Result<(), LoxResult> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    pub fn execute(&self, stmt: &Stmt) -> Result<(), LoxResult> {
        stmt.accept(self)
    }

    pub fn execute_block(
        &self,
        stmts: &Rc<Vec<Rc<Stmt>>>,
        env: Environment,
    ) -> Result<(), LoxResult> {
        let prev_env = self.environment.replace(Rc::new(RefCell::new(env)));

        let ret = stmts.iter().try_for_each(|stmt| self.execute(stmt));

        self.environment.replace(prev_env);

        ret
    }

    pub fn look_up_env(&self, name: &Token) -> Result<Object, LoxResult> {
        if let Ok(o) = self.environment.borrow().borrow().get(name) {
            Ok(o)
        } else {
            self.env_globals.borrow().get(name)
        }
    }
}
