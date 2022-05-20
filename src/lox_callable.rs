use crate::errors::LoxResult;
use crate::interpreter::Interpreter;
use crate::object::Object;

pub trait LoxCallable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxResult>;
    fn arity(&self) -> usize;
}
