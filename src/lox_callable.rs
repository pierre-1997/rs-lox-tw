use crate::interpreter::Interpreter;
use crate::token::Object;

pub trait LoxCallable {
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Object>) -> Object;
    fn arity(&self) -> usize;
}
