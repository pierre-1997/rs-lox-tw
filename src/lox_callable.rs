use crate::errors::LoxError;
use crate::interpreter::Interpreter;
use crate::object::Object;

pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Result<Object, LoxError>;
    fn arity(&self) -> usize;
}
