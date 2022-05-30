use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Index;
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
 * The Interpreter will run through the scanned tokens and interpret it as source code.
 */
pub struct Interpreter {
    /// The current environment of the source code being ran.
    environment: Rc<RefCell<Environment>>,
    /// The top-level global environment of the source code being ran.
    pub env_globals: Rc<RefCell<Environment>>,
    /// The local variables of the source code being ran.
    /// TODO: Refactor into an Environment and references to Tokens, no ?
    locals: HashMap<Token, usize>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    /**
     * Note: This is where `Object::Native` functions are defined in `self.env_globals`.
     */
    pub fn new() -> Self {
        // Instanciate a new empty environment
        let globals = Rc::new(RefCell::new(Environment::new()));

        // Define the `clock()` function as a native one.
        globals.borrow_mut().define(
            "clock".to_string(),
            Object::Native(Rc::new(NativeFunction {
                function: Rc::new(NativeClock {}),
            })),
        );

        // Return a new Interpreter instance
        // NOTE: Shouldn't the global env be enclosed in the env ?
        Interpreter {
            environment: Rc::clone(&globals),
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

    pub fn execute_block(
        &mut self,
        stmts: &[Stmt],
        env: Rc<RefCell<Environment>>,
    ) -> Result<(), LoxResult> {
        let prev_env = self.environment.clone();

        let steps = || -> Result<(), LoxResult> {
            self.environment = env;
            for stmt in stmts {
                self.execute(stmt)?
            }
            Ok(())
        };

        let result = steps();
        self.environment = prev_env;
        result
    }

    pub fn look_up_variable(&self, name: &Token) -> Result<Object, LoxResult> {
        // TODO: Sort out this `self.locals` mess. There must be something strange about it.
        // Try to get it from the environment
        if let Ok(obj) = self.environment.borrow().get(name) {
            return Ok(obj);
        }
        // Try to get it from locals
        if let Some(distance) = self.locals.get(name) {
            Ok(self.environment.borrow().get_at(*distance, name)?)
        }
        // Try to get it from globals
        else {
            self.env_globals.borrow().get(name)
        }
    }

    /**
     * Tells the interpreter that there is the variable `name` that is defined at
     * a the specific `depth`.
     */
    pub fn resolve(&mut self, name: &Token, depth: usize) {
        // Insert the entry (name, depth) in the `self.locals` hashmap
        self.locals.insert(name.clone(), depth);
    }
}

/**
 * Implementation of the expression visitor pattern for the `Interpreter`.
 */
impl ExprVisitor<Object> for Interpreter {
    /**
     * A literal expression is a value: f64, true, false, nil.
     */
    fn visit_literal_expr(&mut self, value: &Option<Object>) -> Result<Object, LoxResult> {
        Ok(value.clone().unwrap())
    }

    /**
     * An unary expression is composed of `-` or `!` followed by an expression: -45.3,
     * !is_function().
     */
    fn visit_unary_expr(&mut self, operator: &Token, right: &Expr) -> Result<Object, LoxResult> {
        // Get the end result of the right expression
        let right = self.evaluate(right)?;

        // '-' or `!`
        match operator.ttype {
            TokenType::Minus => {
                // If the right expression was a number, return its negation
                if let Object::Num(x) = right {
                    Ok(Object::Num(-x))
                } else {
                    // Else, return an error
                    Err(LoxResult::Runtime {
                        token: operator.clone(),
                        error_type: RuntimeErrorType::ExpectedNumberOperand,
                    })
                }
            }
            TokenType::Bang => {
                // Return the boolean negation of the right expression
                Ok(Object::from(!self.is_truthy(right)))
            }
            // If it was neither `-` nor `!`, return an error
            _ => Err(LoxResult::Runtime {
                token: operator.clone(),
                error_type: RuntimeErrorType::UnreachableCode,
            }),
        }
    }

    /**
     * An assign expression is composed of a **known** variable name, a `=` and
     * a value: variable_1 = 45.3.
     *
     * Note: The `self.env_globals` can throw a `EnvironmentErrorType::UnknownVariable`
     * error here.
     */
    fn visit_assign_expr(&mut self, name: &Token, value: &Expr) -> Result<Object, LoxResult> {
        // Evaluate the value
        let value = self.evaluate(value)?;

        // Try to get the known variable from the locally defined ones.
        let distance = self.locals.index(name);

        // If we found it, reassign it to the evaluated value
        if distance > &0 {
            self.environment
                .borrow_mut()
                .assign_at(*distance, name.clone(), value.clone());
        }
        // Else, try to assign it in the globally known variables
        else {
            self.env_globals.borrow_mut().assign(name, value.clone())?;
        }

        Ok(value)
    }

    /**
     * A group expression is
     */
    fn visit_grouping_expr(&mut self, expression: &Expr) -> Result<Object, LoxResult> {
        self.evaluate(expression)
    }

    /**
     * A binary expression contains a left and a right expression separated by an operator.
     * Example: 3 - 4,
     */
    fn visit_binary_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxResult> {
        // Evaluate the left and right expressions
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        // Check the operator
        match operator.ttype {
            // `-`
            TokenType::Minus => {
                // Check that both left and right expressions are numbers
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left - right));
                    }
                }
                // If not, return an error
                Err(LoxResult::Runtime {
                    token: operator.clone(),
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            // `/`
            TokenType::Slash => {
                // Check that both left and right expressions are numbers
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left / right));
                    }
                }
                // If not, return an error
                Err(LoxResult::Runtime {
                    token: operator.clone(),
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            // `*`
            TokenType::Star => {
                // Check that both left and right expressions are numbers
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left * right));
                    }
                }
                // If not, return an error
                Err(LoxResult::Runtime {
                    token: operator.clone(),
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            // `+`
            TokenType::Plus => {
                // Check if both left and right expressions are numbers
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left + right));
                    }
                }
                // Check if both left and right expressions are strings
                if let Object::Str(left) = left {
                    if let Object::Str(right) = right {
                        let mut s = left;
                        s.push_str(&right);
                        return Ok(Object::from(s));
                    }
                }
                // TODO: Specific error for when 2 different type (a string and a number)
                // If neither, return an error
                Err(LoxResult::Runtime {
                    token: operator.clone(),
                    error_type: RuntimeErrorType::ExpectedAddableOperands,
                })
            }

            // Comparison operators
            // `>`
            TokenType::Greater => {
                // Check if both left and right expressions are numbers
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left > right));
                    }
                }
                // If not, return an error
                Err(LoxResult::Runtime {
                    token: operator.clone(),
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            // `>=`
            TokenType::GreaterEqual => {
                // Check if both left and right expressions are numbers
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left >= right));
                    }
                }
                // If not, return an error
                Err(LoxResult::Runtime {
                    token: operator.clone(),
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            // `<`
            TokenType::Less => {
                // Check if both left and right expressions are numbers
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left < right));
                    }
                }
                // If not, return an error
                Err(LoxResult::Runtime {
                    token: operator.clone(),
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            // `<=`
            TokenType::LessEqual => {
                // Check if both left and right expressions are numbers
                if let Object::Num(left) = left {
                    if let Object::Num(right) = right {
                        return Ok(Object::from(left <= right));
                    }
                }
                // If not, return an error
                Err(LoxResult::Runtime {
                    token: operator.clone(),
                    error_type: RuntimeErrorType::ExpectedNumberOperands,
                })
            }

            // `!=`
            TokenType::BangEqual => Ok(Object::from(left != right)),

            // `==`
            TokenType::EqualEqual => Ok(Object::from(left == right)),

            // Error otherwise
            _ => Err(LoxResult::Runtime {
                token: operator.clone(),
                error_type: RuntimeErrorType::UnreachableCode,
            }),
        }
    }

    /**
     * Called when trying to access a **known** variable's value
     *
     * Note: Calls `self.look_up_variable()` which can throw a
     * `EnvironmentErrorType::UnknownVariable` error.
     */
    fn visit_variable_expr(&mut self, name: &Token) -> Result<Object, LoxResult> {
        // Try getting the variable's value
        self.look_up_variable(name)
    }

    /**
     * A logical expression contains a left and a right expressions  separated by a
     * logical operator (e.g. logical `or` or `and`).
     *
     * Example: a || b, c && d
     */
    fn visit_logical_expr(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Object, LoxResult> {
        // Evaluate the left expression
        let left = self.evaluate(left)?;

        // If we are on a `or`
        if operator.ttype == TokenType::Or {
            // Return the left expression if it is truthy already
            // (e.g. true || <something>)
            if self.is_truthy(left.clone()) {
                return Ok(left);
            }
        }
        // Else if we are on a `and` the left expression is already not truthy,
        // return it (e.g. false && <something>)
        else if !self.is_truthy(left.clone()) {
            return Ok(left);
        }

        // Otherwise, return the evaluated right expression
        self.evaluate(right)
    }

    /**
     * This evaluates the next source code part as a function (native or defined) or a
     * class call. Returns the call's result as an object or Nil if it did not return
     * anything.
     */
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

    /**
     * Attempts to get an instance's member (e.g. a property, a field or a method).
     *
     * Note: If the `object` expression does not evaluate into an `Object::Instance`,
     * this function will return a `RuntimeErrorType::InvalidObjectProperty` error.
     */
    fn visit_get_expr(&mut self, object: &Expr, name: &Token) -> Result<Object, LoxResult> {
        // Evaluate the given expression
        let obj = self.evaluate(object)?;
        // Check that its evaluation gave an instance object
        if let Object::Instance(ref instance) = obj {
            // If so, returns the attempt of getting a member from it.
            instance.get(name, &obj)
        } else {
            // If it was not an instance, return an error
            Err(LoxResult::Runtime {
                token: name.clone(),
                error_type: RuntimeErrorType::InvalidObjectProperty,
            })
        }
    }

    /**
     * Attempts to set an instance's member (e.g. a property, a field or a method).
     *
     * Note: If the `object` expression does not evaluate into an `Object::Instance`,
     * this function will return a `RuntimeErrorType::InvalidObjectProperty` error.
     */
    fn visit_set_expr(
        &mut self,
        object: &Expr,
        name: &Token,
        value: &Expr,
    ) -> Result<Object, LoxResult> {
        // Evaluate the given expression
        let obj = self.evaluate(object)?;
        // Check that its evaluation gave an instance object
        if let Object::Instance(instance) = obj {
            // If so, evaluate the given value expression and set it to the instance
            let val = self.evaluate(value)?;
            instance.set(name, val.clone());
            Ok(val)
        } else {
            // If it was not an instance, return an error
            Err(LoxResult::Runtime {
                token: name.clone(),
                error_type: RuntimeErrorType::InvalidObjectProperty,
            })
        }
    }

    /**
     * Function called when trying to access `this` variable.
     */
    fn visit_this_expr(&mut self, keyword: &Token) -> Result<Object, LoxResult> {
        // Simply lookup a `this` variable as it should currently be defined locally
        self.look_up_variable(keyword)
    }
}

/**
 * Implementation of the statement visitor pattern for the `Interpreter`.
 */
impl StmtVisitor<()> for Interpreter {
    /**
     * Evaluates an expression.
     *
     * Note: If it evaluated to an error, print it.
     */
    fn visit_expression_stmt(&mut self, expression: &Expr) -> Result<(), LoxResult> {
        if let Err(e) = self.evaluate(expression) {
            eprintln!("{}", e);
        }

        Ok(())
    }

    /**
     * Evaluate a print expression and prints its outcome.
     */
    fn visit_print_stmt(&mut self, expression: &Expr) -> Result<(), LoxResult> {
        let value = self.evaluate(expression)?;
        println!("{}", value);

        Ok(())
    }

    /**
     * Evaluate a variable declaration expression. The value to be set is optional.
     *
     * Example: var a = 3
     */
    fn visit_var_stmt(
        &mut self,
        name: &Token,
        initializer: &Option<Expr>,
    ) -> Result<(), LoxResult> {
        // Defaults all variable declarations to Nil
        let mut value = Object::Nil;
        // If there was a given initializer value, evaluate and set it in `value`
        if initializer.is_some() {
            value = self.evaluate(initializer.as_ref().unwrap())?;
        }
        // Define the newly declared variable in the current environment
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), value);

        Ok(())
    }

    /**
     * Executes a block (scope) of source code in a new environment.
     */
    fn visit_block_stmt(&mut self, statements: &[Stmt]) -> Result<(), LoxResult> {
        // Create a new environment for the scope
        let env = Environment::from_enclosing(Rc::clone(&self.environment));
        // Execute the statements of the block in the new environment
        self.execute_block(statements, Rc::new(RefCell::new(env)))
    }

    /**
     * Executes an if statement composed of a condition expression, a then branch and
     * an optional else branch.
     */
    fn visit_if_stmt(
        &mut self,
        condition: &Expr,
        then_branch: &Stmt,
        else_branch: &Option<Stmt>,
    ) -> Result<(), LoxResult> {
        // Evaluate the condition expression.
        let condition_value = self.evaluate(condition)?;
        // If the condition evaluates to true, execute the code of the then branch.
        if self.is_truthy(condition_value) {
            self.execute(then_branch)?;
        } else if let Some(else_branch) = else_branch {
            // Else if there was a given else branch, execute its code.
            self.execute(else_branch)?;
        }

        Ok(())
    }

    /**
     * This function executes a return statement using an optional value to return.
     * Defaults to `Object::Nil`.
     *
     * Note: This function will use the `LoxResult::ReturnValue` variant in order to
     * return an actual value instead of an error.
     */
    fn visit_return_stmt(
        &mut self,
        _keyword: &Token,
        value: &Option<Expr>,
    ) -> Result<(), LoxResult> {
        // If we were given a value return it
        if let Some(v) = &value {
            Err(LoxResult::ReturnValue {
                value: self.evaluate(v)?,
            })
        }
        // Otherwise, return `Object::Nil`
        else {
            Err(LoxResult::ReturnValue { value: Object::Nil })
        }
    }

    /**
     * Exectute a while statement containing a condition and a body.
     * Note: we have to transform the given `while {}`
     * `
     * while <condition> {
     *     <body>
     * }
     * `
     *
     * into a Rust's `loop {}`
     * `
     * loop {
     *    if !<condition> { break; }
     *    <body>
     * }
     * `
     * here.
     */
    fn visit_while_stmt(&mut self, condition: &Expr, body: &Stmt) -> Result<(), LoxResult> {
        loop {
            // Evaluate the condition
            let condition_value = self.evaluate(condition)?;
            // If the evaluated condition is false, break out of the loop
            if !self.is_truthy(condition_value) {
                break;
            }

            // Execute the body
            self.execute(body)?;
        }

        Ok(())
    }

    /**
     * Defines a new function in the current environment. A function is composed of
     * a name, an array of parameters and an array of statements that compose its body.
     */
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
            closure: Rc::clone(&self.environment),
        }));

        // Define the function in the current environment
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), function);

        Ok(())
    }

    /**
     * Function called when interpretting a class declaration statement.
     */
    fn visit_class_stmt(&mut self, name: &Token, methods: &[Stmt]) -> Result<(), LoxResult> {
        // Define the class in the environment as a null object for now
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), Object::Nil);

        // Interpret each defined class method into a `LoxFunction` object
        let mut class_methods: HashMap<String, LoxFunction> = HashMap::new();
        for method in methods {
            // Extract the name, body and param of the method
            if let Stmt::Function { name, params, body } = method {
                let function = LoxFunction {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: Rc::clone(&self.environment),
                };

                // Put a `LoxFunction` struct into the hashmap
                class_methods.insert(name.lexeme.clone(), function);
            } else {
                unreachable!()
            };
        }

        // Instanciate a new `Object::Class` containing the name of the classs and its methods
        let class = Object::Class(Rc::new(LoxClass {
            name: name.lexeme.clone(),
            methods: class_methods,
        }));

        // Set the previously declared object in the environment as the newly created class object.
        self.environment.borrow_mut().assign(name, class)?;

        Ok(())
    }
}
