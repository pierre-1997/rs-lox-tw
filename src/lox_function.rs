use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::environment::Environment;
use crate::errors::LoxResult;
use crate::interpreter::Interpreter;
use crate::lox_callable::LoxCallable;
use crate::lox_class::LoxClass;
use crate::object::Object;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Clone)]
pub struct LoxFunction {
    pub name: Token,
    // TODO: Refactor into Rc<Vec<Token>>
    /// Contains the list of parameters/arguments of the function.
    pub params: Vec<Token>,
    // TODO: Refactor into Rc<Vec<Stmt>>
    /// Contains the list of statements that compose the function's body.
    pub body: Vec<Stmt>,
    /// Environment used by the function itself. When defined, takes the values from the
    /// surrounding environment.
    pub closure: Rc<RefCell<Environment>>,
    /// Tells if this is a class's `init()` function
    pub is_init_function: bool,
}

impl LoxFunction {
    /**
     * Binds the function to a runtime object instance (e.g. a class)
     */
    pub fn bind(&self, instance: &Object) -> LoxFunction {
        // Create a new environment that contains the current function's one
        let new_env = RefCell::new(Environment::from_enclosing(Rc::clone(&self.closure)));

        // Define `this` in that new environment
        new_env
            .borrow_mut()
            .define("this".to_string(), instance.clone());

        // Return a new `LoxFunction` that just have this environment changed
        Self {
            name: self.name.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
            closure: Rc::new(new_env),
            is_init_function: self.is_init_function,
        }
    }
}

impl fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{self}")
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
        _class: Option<Rc<LoxClass>>,
    ) -> Result<Object, LoxResult> {
        // Create a new environment for the function's scope
        let mut env = Environment::from_enclosing(Rc::clone(&self.closure));

        // Define the function's arguments in the function's env
        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            env.define(param.lexeme.clone(), arg.clone());
        }

        // Handle the execution's return
        match interpreter.execute_block(&self.body, Rc::new(RefCell::new(env))) {
            // Returned a value
            Err(LoxResult::ReturnValue { value }) => {
                // If we're in a class's init() function, return `this`
                if self.is_init_function {
                    return self.closure.borrow_mut().get_at(
                        0,
                        &Token {
                            ttype: TokenType::This,
                            lexeme: "this".to_string(),
                            ..Default::default()
                        },
                    );
                }
                // Else return the value
                Ok(value)
            }
            // Returned an error
            Err(e) => Err(e),
            // Returned nothing, force return `Object::Nil` in a regular function and
            // `this` in an init() flass function.
            Ok(_) => {
                if self.is_init_function {
                    return self.closure.borrow_mut().get_at(
                        0,
                        &Token {
                            ttype: TokenType::This,
                            lexeme: "this".to_string(),
                            ..Default::default()
                        },
                    );
                }
                Ok(Object::Nil)
            }
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<fn {}({})>",
            self.name.lexeme,
            self.params
                .iter()
                .map(|x| x.lexeme.clone())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
