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

#[derive(Clone)]
pub struct LoxFunction {
    pub name: Token,
    // TODO: Refactor into Rc<Vec<Token>>
    pub params: Vec<Token>,
    // TODO: Refactor into Rc<Vec<Stmt>>
    pub body: Vec<Stmt>,
    pub closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn bind(&self, instance: Object) -> LoxFunction {
        let mut new_env = Environment::from_enclosing(Rc::clone(&self.closure));

        new_env.define("this".to_string(), instance);

        Self {
            name: self.name.clone(),
            params: self.params.clone(),
            body: self.body.clone(),
            closure: Rc::new(RefCell::new(new_env)),
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
        let mut env = Environment::from_enclosing(Rc::clone(&self.closure));

        for (param, arg) in self.params.iter().zip(arguments.iter()) {
            env.define(param.lexeme.clone(), arg.clone());
        }

        // Handle the execution's return
        match interpreter.execute_block(&self.body, Rc::new(RefCell::new(env))) {
            Err(LoxResult::ReturnValue { value }) => Ok(value),
            Err(e) => Err(e),
            Ok(_) => Ok(Object::Nil),
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
