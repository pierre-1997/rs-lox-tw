use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::errors::LoxResult;
use crate::interpreter::Interpreter;
use crate::lox_callable::LoxCallable;
use crate::lox_function::LoxFunction;
use crate::lox_instance::LoxInstance;
use crate::object::Object;

/**
 * This structure represents a Lox class. It contains the name of the class as well as the list of
 * its defined methods.
 */
#[derive(Debug)]
pub struct LoxClass {
    /// The name of the class.
    pub name: String,
    /// A map of defined functions for this class.
    pub methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    /**
     * Function used in order to retrieve a defined method of the current class.
     *
     * Note: Returns `None` if not found.
     */
    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        if let Some(method) = self.methods.get(name) {
            return Some(method.clone());
        }

        None
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "<class {}", self.name)?;
        if !self.methods.is_empty() {
            for (name, obj) in &self.methods {
                writeln!(f, "- this.{} = {}", name, obj)?;
            }
        } else {
            writeln!(f, "Methods: None")?;
        }
        writeln!(f, ">")?;
        Ok(())
    }
}

impl LoxCallable for LoxClass {
    /**
     * When calling the class itself, behave as a class constructor and return a newly created
     * instance of that class.
     */
    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
        class: Option<Rc<LoxClass>>,
    ) -> Result<Object, LoxResult> {
        let instance = LoxInstance::new(&class.unwrap());
        Ok(Object::Instance(Rc::new(instance)))
    }

    fn arity(&self) -> usize {
        0
    }
}
