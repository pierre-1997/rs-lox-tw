use std::rc::Rc;

use crate::errors::LoxResult;
use crate::interpreter::Interpreter;
use crate::lox_callable::LoxCallable;
use crate::lox_class::LoxClass;
use crate::object::Object;

pub struct NativeClock;

impl LoxCallable for NativeClock {
    fn call(
        &self,
        _: &mut Interpreter,
        _: Vec<Object>,
        class: Option<Rc<LoxClass>>,
    ) -> Result<Object, LoxResult> {
        Ok(Object::Num(
            chrono::offset::Local::now().timestamp_millis() as f64 / 1000.0,
        ))
    }

    fn arity(&self) -> usize {
        0
    }
}
