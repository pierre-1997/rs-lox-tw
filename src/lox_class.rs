use std::fmt;
use std::rc::Rc;

use crate::errors::LoxResult;
use crate::interpreter::Interpreter;
use crate::lox_callable::LoxCallable;
use crate::lox_instance::LoxInstance;
use crate::object::Object;

#[derive(Debug)]
pub struct LoxClass {
    pub name: String,
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl LoxCallable for LoxClass {
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
        class: Option<Rc<LoxClass>>,
    ) -> Result<Object, LoxResult> {
        /*
        if class.is_none() {
            return Err();
        }
        */

        let instance = LoxInstance::new(&class.unwrap());
        Ok(Object::Instance(Rc::new(instance)))
    }

    fn arity(&self) -> usize {
        0
    }
}
