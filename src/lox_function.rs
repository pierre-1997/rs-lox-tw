use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::environment::Environment;
use crate::errors::LoxError;
use crate::interpreter::Interpreter;
use crate::lox_callable::LoxCallable;
use crate::object::Object;
use crate::stmt::FunctionStmt;
use crate::stmt::Stmt;
use crate::token::Token;

pub struct LoxFunction {
    pub name: Token,
    pub params: Rc<Vec<Token>>,
    pub body: Rc<Vec<Rc<Stmt>>>,
}

impl fmt::Debug for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{self}")
    }
}

impl Clone for LoxFunction {
    fn clone(&self) -> Self {
        Self {
            name: self.name.dup(),
            params: Rc::clone(&self.params),
            body: Rc::clone(&self.body),
        }
    }
}

impl LoxFunction {
    /*
    fn new(declaration: &FunctionStmt) -> Self {
        Self {
            name: declaration.name.dup(),
            params: Rc::clone(&declaration.params),
            body: Rc::clone(&declaration.body),
        }
    }
    */
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name.ttype == other.name.ttype
            && Rc::ptr_eq(&self.params, &other.params)
            && Rc::ptr_eq(&self.body, &other.body)
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxError> {
        let mut env = Environment::from_enclosing(interpreter.env_globals.clone());

        for i in 0..self.params.len() {
            env.define(
                self.params.get(i).unwrap().lexeme.clone(),
                arguments.get(i).unwrap().clone(),
            );
        }

        interpreter.execute_block(&self.body, env)?;
        Ok(Object::Nil)
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme)
    }
}
