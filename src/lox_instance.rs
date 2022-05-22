use std::collections::HashMap;
use std::fmt;

use std::rc::Rc;

use crate::errors::{LoxResult, RuntimeErrorType};
use crate::lox_class::LoxClass;
use crate::object::Object;
use crate::token::Token;

#[derive(Debug)]
pub struct LoxInstance {
    class: Rc<LoxClass>,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(class: &Rc<LoxClass>) -> Self {
        LoxInstance {
            class: Rc::clone(class),
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxResult> {
        // Check if the variable exists locally
        if let Some(field) = self.fields.get(&name.lexeme) {
            return Ok(field.clone());
        }

        Err(LoxResult::Runtime {
            token: name.clone(),
            error_type: RuntimeErrorType::UndefinedProperty,
        })
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<instance of {}>", self.class)
    }
}
