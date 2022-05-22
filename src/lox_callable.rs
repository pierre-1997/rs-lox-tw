use std::rc::Rc;

use crate::errors::LoxResult;
use crate::interpreter::Interpreter;
use crate::lox_class::LoxClass;
use crate::object::Object;

pub trait LoxCallable {
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
        class: Option<Rc<LoxClass>>,
    ) -> Result<Object, LoxResult>;
    fn arity(&self) -> usize;
}
