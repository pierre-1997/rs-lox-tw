use crate::errors::{EnvironmentErrorType, LoxError};
use crate::token::{Object, Token};

use std::collections::HashMap;

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, obj: Object) {
        self.values.insert(name, obj);
    }

    pub fn get(&self, name: Token) -> Result<Object, LoxError> {
        match self.values.get(&name.lexeme) {
            Some(v) => Ok(v.clone()),
            None => Err(LoxError::EnvironmentError {
                error_type: EnvironmentErrorType::UnknownVariable,
            }),
        }
    }
}
