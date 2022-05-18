use crate::errors::LoxError;
use crate::interpreter::Interpreter;
use crate::lox_callable::LoxCallable;
use crate::object::Object;

pub struct NativeClock;

impl LoxCallable for NativeClock {
    fn call(&self, _: &Interpreter, _: Vec<Object>) -> Result<Object, LoxError> {
        Ok(Object::Num(
            chrono::offset::Local::now().timestamp_millis() as f64 / 1000.0,
        ))
    }

    fn arity(&self) -> usize {
        0
    }
}
