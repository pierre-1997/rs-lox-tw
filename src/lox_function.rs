use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::environment::Environment;
use crate::errors::LoxResult;
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
    closure: Rc<RefCell<Environment>>,
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
            closure: Rc::clone(&self.closure),
        }
    }
}

impl LoxFunction {
    pub fn new(declaration: &FunctionStmt, closure: &Rc<RefCell<Environment>>) -> Self {
        Self {
            name: declaration.name.dup(),
            params: declaration.params.clone(),
            body: declaration.body.clone(),
            closure: Rc::clone(closure),
        }
    }
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name.ttype == other.name.ttype
            && Rc::ptr_eq(&self.params, &other.params)
            && Rc::ptr_eq(&self.body, &other.body)
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        let mut env = Environment::from_enclosing(Rc::clone(&self.closure));

        for i in 0..self.params.len() {
            env.define(
                self.params.get(i).unwrap().lexeme.clone(),
                arguments.get(i).unwrap().clone(),
            );
        }

        if let Err(LoxResult::ReturnValue { value }) = interpreter.execute_block(&self.body, env) {
            return Ok(value);
        }

        Ok(Object::Nil)
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