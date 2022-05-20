use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::environment::Environment;
use crate::errors::LoxResult;
use crate::interpreter::Interpreter;
use crate::lox_callable::LoxCallable;
use crate::object::Object;
use crate::stmt::FunctionStmt;

pub struct LoxFunction {
    declaration: Rc<FunctionStmt>,
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
            declaration: Rc::clone(&self.declaration),
            closure: Rc::clone(&self.closure),
        }
    }
}

impl LoxFunction {
    pub fn new(declaration: &Rc<FunctionStmt>, closure: &Rc<RefCell<Environment>>) -> Self {
        Self {
            declaration: Rc::clone(declaration),
            closure: Rc::clone(closure),
        }
    }
}

impl PartialEq for LoxFunction {
    fn eq(&self, other: &Self) -> bool {
        self.declaration.name == other.declaration.name
            && Rc::ptr_eq(&self.declaration.params, &other.declaration.params)
            && Rc::ptr_eq(&self.declaration.body, &other.declaration.body)
    }
}

impl LoxCallable for LoxFunction {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxResult> {
        let mut env = Environment::from_enclosing(Rc::clone(&self.closure));

        for i in 0..self.declaration.params.len() {
            env.define(
                self.declaration.params.get(i).unwrap().lexeme.clone(),
                arguments.get(i).unwrap().clone(),
            );
        }

        if let Err(LoxResult::ReturnValue { value }) =
            interpreter.execute_block(&self.declaration.body, env)
        {
            return Ok(value);
        }

        Ok(Object::Nil)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<fn {}({})>",
            self.declaration.name.lexeme,
            self.declaration
                .params
                .iter()
                .map(|x| x.lexeme.clone())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
