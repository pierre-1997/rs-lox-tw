use crate::errors::{EnvironmentErrorType, LoxError};
use crate::token::{Object, Token};

use std::collections::HashMap;


pub struct Environment {
    enclosing: Option<Box<Environment>>,
    values: HashMap<String, Object>,
}

impl Environment {
    /// Useless
    pub fn new() -> Self {
        Environment { enclosing: None, values: HashMap::new() }
    }

    /// Useless too ?
    pub fn from_enclosing(env: Environment) -> Self {
        Environment {
            enclosing: Some(Box::new(env)),
            values: HashMap::new()
        }
    }

    /**
     * Inserts a key-value pair in the global HashMap storage.
     */
    pub fn define(&mut self, name: String, obj: Object) {
        self.values.insert(name, obj);
    }

    /**
     * Gets a value using its key name from the storage HashMap.
     *
     * Note: Throws an error if the key does not exist.
     */
    pub fn get(&self, token: Token) -> Result<Object, LoxError> {
        // Check if the variable exists locally
        if let Some(v) =  self.values.get(&token.lexeme) {
            return Ok(v.clone());
        }

        // If we have an enclosing environment, check inside too
        if self.enclosing.is_some() {
            return self.enclosing.as_ref().unwrap().get(token);
        }

        // Else, throw an error
        Err(LoxError::Environment {
            error_type: EnvironmentErrorType::UnknownVariable,
            msg: format!(
                "{} -> No such variable '{}'.",
                token.location(),
                token.lexeme
            ),
        })
    }

    pub fn assign(&mut self, token: Token, value: Object) -> Result<(), LoxError> {
        // Try inserting in the local variables
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            self.values.entry(token.lexeme.clone())
        {
            e.insert(value);
            return Ok(());
        }

        // If we have an enclosing, check if we can insert into it
        if self.enclosing.is_some() {
            return self.enclosing.as_mut().unwrap().assign(token, value);
        }

        // Otherwise, throw an error because the variable we tried to assign does not exist
        Err(LoxError::Environment {
            error_type: EnvironmentErrorType::UnknownVariable,
            msg: format!(
                "Cannot assign value to unknown variable '{}'.",
                token.lexeme
            ),
        })
    }
}


