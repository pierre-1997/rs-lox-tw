use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::errors::{EnvironmentErrorType, LoxResult};
use crate::object::Object;
use crate::token::Token;

#[derive(Debug)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in &self.values {
            writeln!(f, "{} = {}", k, v)?
        }

        if self.enclosing.is_some() {
            writeln!(
                f,
                "Enclosing: {}",
                self.enclosing.as_ref().unwrap().borrow()
            )?
        } else {
            writeln!(f, "Enclosing: false.")?
        }

        Ok(())
    }
}

impl Environment {
    /// Useless
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn from_enclosing(env: Rc<RefCell<Environment>>) -> Self {
        Environment {
            enclosing: Some(env),
            values: HashMap::new(),
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
    pub fn get(&self, token: &Token) -> Result<Object, LoxResult> {
        // Check if the variable exists locally
        if let Some(v) = self.values.get(&token.lexeme) {
            return Ok(v.clone());
        }

        // If we have an enclosing environment, check inside too (recursive)
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(token);
        }

        // Else, throw an error
        Err(LoxResult::Environment {
            error_type: EnvironmentErrorType::UnknownVariable,
            msg: format!(
                "{} -> No such variable '{}'.",
                token.location(),
                token.lexeme
            ),
        })
    }

    pub fn get_at(&self, distance: usize, name: &Token) -> Result<Object, LoxResult> {
        self.ancestor(distance).borrow().get(name)
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        let parent = self.enclosing.clone().expect("No ancestor at depth 1.");
        let mut env = Rc::clone(&parent);

        for i in 0..distance {
            let parent = env
                .borrow()
                .enclosing
                .clone()
                .unwrap_or_else(|| panic!("No ancestor at depth {i}."));

            env = Rc::clone(&parent);
        }

        env
    }

    pub fn assign(&mut self, token: &Token, value: Object) -> Result<(), LoxResult> {
        // Try inserting in the local variables
        if let Entry::Occupied(mut e) = self.values.entry(token.lexeme.clone()) {
            e.insert(value);
            return Ok(());
        }

        // If we have an enclosing, check if we can insert into it
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(token, value);
        }

        // Otherwise, throw an error because the variable we tried to assign does not exist
        Err(LoxResult::Environment {
            error_type: EnvironmentErrorType::UnknownVariable,
            msg: format!(
                "Cannot assign value to unknown variable '{}'.",
                token.lexeme
            ),
        })
    }

    pub fn assign_at(&mut self, distance: usize, name: Token, value: Object) {
        self.ancestor(distance)
            .borrow_mut()
            .define(name.lexeme, value);
    }
}
