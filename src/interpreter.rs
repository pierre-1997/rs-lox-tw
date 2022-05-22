use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, Index};
use std::rc::Rc;

use crate::environment::Environment;
use crate::errors::{LoxResult, RuntimeErrorType};
use crate::expr::*;
use crate::lox_callable::LoxCallable;
use crate::lox_class::LoxClass;
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
    locals: HashMap<Token, usize>,
}

impl ExprVisitor<Object> for Interpreter {
    fn visit_literal_expr(&mut self, value: &Option<Object>) -> Result<Object, LoxResult> {
        Ok(value.clone().unwrap())
    }

    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, LoxResult> {
        let right = self.evaluate(right)?;

        match operator.ttype {
            TokenType::Minus => {
                if let Object::Num(x) = right {
                    Ok(Object::Num(-x))
                } else {
                    Err(LoxResult::Runtime {
                        token: operator.clone(),
                        error_type: RuntimeErrorType::ExpectedNumberOperand,
                    })
                }
            }
            TokenType::Bang => Ok(Object::from(!self.is_truthy(right))),
            _ => Err(LoxResult::Runtime {
                token: operator.clone(),
                error_type: RuntimeErrorType::UnreachableCode,
            }),
        }
    }

    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, LoxResult> {
        let value = self.evaluate(value)?;

        let distance = self.locals.index(name);

        if distance > &0 {
            self.environment.borrow().borrow_mut().assign_at(
                *distance,
                name.clone(),
                value.clone(),
            );
        } else {
            self.env_globals.borrow_mut().assign(name, value.clone())?;
        }

        Ok(value)
    }

    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Object, LoxResult> {
        self.evaluate(expression)
    }

    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxResult> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.ttype {
            TokenType::Minus => {
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left - right));
                    }
                }

                Err(LoxResult::Runtime {
                    token: operator.clone(),
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
                    token: operator.clone(),
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
                    token: operator.clone(),
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
                    token: operator.clone(),
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
                    token: operator.clone(),
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
                    token: operator.clone(),
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
                    token: operator.clone(),
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
                    token: operator.clone(),
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            //Handle '!='
            TokenType::BangEqual => Ok(Object::from(left != right)),

            //Handle '=='
            TokenType::EqualEqual => Ok(Object::from(left == right)),

            _ => Err(LoxResult::Runtime {
                token: operator.clone(),
                error_type: RuntimeErrorType::UnreachableCode,
            }),
        }
    }

    fn visit_variable_expr(&mut self, name: &Token) -> Result<Object, LoxResult> {
        self.look_up_variable(name)
    }

    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxResult> {
        let left = self.evaluate(left)?;

        if operator.ttype == TokenType::Or {
            if self.is_truthy(left.clone()) {
                return Ok(left);
            }
        } else if !self.is_truthy(left.clone()) {
            return Ok(left);
        }

        self.evaluate(right)
    }

    fn visit_call_expr(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &[Expr],
    ) -> Result<Object, LoxResult> {
        // Get the expression's callee
        let callee = self.evaluate(callee)?;

        // Optional vector of arguments
        let mut call_args: Vec<Object> = Vec::new();
        // Evaluate each calling argument
        for argument in arguments {
            call_args.push(self.evaluate(argument)?);
        }

        // Try to interpret the callee as a callable object (e.g function or class)
        let (called_function, called_class): (Rc<dyn LoxCallable>, Option<Rc<LoxClass>>) =
            match callee {
                // Check for native function
                Object::Native(native) => (native.function.clone(), None),
                // Check for defined function
                Object::Function(function) => (function, None),
                // Check for define classes
                //
                Object::Class(class) => {
                    let called_class = Rc::clone(&class);
                    (class, Some(called_class))
                }
                // Otherwise, this is not a callable object type, return an error.
                _ => {
                    return Err(LoxResult::Runtime {
                        token: paren.clone(),
                        error_type: RuntimeErrorType::InvalidCallObjectType,
                    });
                }
            };

        // Check called function's arity and return error if incorrect
        if arguments.len() != called_function.arity() {
            return Err(LoxResult::Runtime {
                token: paren.clone(),
                error_type: RuntimeErrorType::InvalidArgsCount,
            });
        }

        // Return the function's call result
        called_function.call(self, call_args, called_class)
    }

    fn visit_get_expr(&mut self, object: &Expr, name: &Token) -> Result<Object, LoxResult> {
        let obj = self.evaluate(object)?;

        if let Object::Instance(instance) = obj {
            return instance.get(name);
        }

        Err(LoxResult::Runtime {
            token: name.clone(),
            error_type: RuntimeErrorType::InvalidObjectProperty,
        })
    }

    fn visit_set_expr(&mut self, object: &Expr, name: &Token, value: &Expr) -> Result<Object, LoxResult> {
        let obj = self.evaluate(object)?;

        if let Object::Instance(instance) = obj {
            let val = self.evaluate(value)?;
            instance.set(name, val.clone());
            Ok(val)
        } else {
            Err(LoxResult::Runtime {
                token: name.clone(),
                error_type: RuntimeErrorType::InvalidObjectProperty,
            })
        }
    }
}

impl StmtVisitor<()> for Interpreter {
    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), LoxResult> {
        if let Err(e) = self.evaluate(expression) {
            eprintln!("{}", e);
        }

        Ok(())
    }

    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<(), LoxResult> {
        let value = self.evaluate(expression)?;
        println!("{}", value);

        Ok(())
    }

    fn visit_var_stmt(
        &mut self,
        name: &Token,
        initializer: &Option<Expr>,
    ) -> Result<(), LoxResult> {
        let mut value = Object::Nil;

        if initializer.is_some() {
            value = self.evaluate(initializer.as_ref().unwrap())?;
        }

        self.environment
            .borrow()
            .borrow_mut()
            .define(name.lexeme.clone(), value);

        Ok(())
    }

    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> Result<(), LoxResult> {
        let env = Environment::from_enclosing(self.environment.borrow().clone());
        self.execute_block(statements, env)
    }

    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Stmt>,
    ) -> Result<(), LoxResult> {
        let condition_value = self.evaluate(condition)?;
        if self.is_truthy(condition_value) {
            self.execute(then_branch)?;
        } else if let Some(else_branch) = else_branch {
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
    fn visit_return_stmt(
        &mut self,
        _keyword: &Token,
        value: &Option<Expr>,
    ) -> Result<(), LoxResult> {
        if let Some(v) = &value {
            Err(LoxResult::ReturnValue {
                value: self.evaluate(v)?,
            })
        } else {
            Err(LoxResult::ReturnValue { value: Object::Nil })
        }
    }

    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), LoxResult> {
        loop {
            let condition_value = self.evaluate(condition)?;
            if !self.is_truthy(condition_value) {
                break;
            }

            self.execute(body)?;
        }

        Ok(())
    }

    fn visit_function_stmt(
        &mut self,
        name: &Token,
        params: &[Token],
        body: &[Stmt],
    ) -> Result<(), LoxResult> {
        // Instanciate a new function object using its statement
        let function = Object::Function(Rc::new(LoxFunction {
            name: name.clone(),
            params: params.to_vec(),
            body: body.to_vec(),
            closure: Rc::clone(self.environment.borrow().deref()),
        }));

        // Define the function in the current environment
        self.environment
            .borrow_mut()
            .borrow_mut()
            .define(name.lexeme.clone(), function);

        Ok(())
    }

    fn visit_class_stmt(&mut self, name: &Token, _methods: &[Stmt]) -> Result<(), LoxResult> {
        self.environment
            .borrow()
            .borrow_mut()
            .define(name.lexeme.clone(), Object::Nil);

        let class = Object::Class(Rc::new(LoxClass {
            name: name.lexeme.clone(),
        }));

        self.environment.borrow().borrow_mut().assign(name, class)?;

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
            locals: HashMap::new(),
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Object, LoxResult> {
        expr.accept(self)
    }

    pub fn is_truthy(&self, obj: Object) -> bool {
        !(obj == Object::Nil || obj == Object::False)
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), LoxResult> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxResult> {
        stmt.accept(self)
    }

    pub fn execute_block(&mut self, stmts: &[Stmt], env: Environment) -> Result<(), LoxResult> {
        let prev_env = self.environment.replace(Rc::new(RefCell::new(env)));

        let ret = stmts.iter().try_for_each(|stmt| self.execute(stmt));

        self.environment.replace(prev_env);

        ret
    }

    pub fn look_up_variable(&self, name: &Token) -> Result<Object, LoxResult> {
        if let Some(distance) = self.locals.get(name) {
            Ok(self.environment.borrow().borrow().get_at(*distance, name)?)
        } else {
            self.env_globals.borrow().get(name)
        }
    }

    pub fn resolve(&mut self, name: &Token, depth: usize) {
        self.locals.insert(name.clone(), depth);
    }
}
