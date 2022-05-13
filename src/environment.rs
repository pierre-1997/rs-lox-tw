use crate::errors::{EnvironmentErrorType, LoxErrors};
use crate::token::{Object, Token};

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
lazy_static! {
    static ref VALUES: Mutex<HashMap<String, Object>> = Mutex::new(HashMap::new());
}

pub struct Environment {}

impl Environment {
    pub fn new() -> Self {
        Environment {}
    }

    pub fn define(&self, name: String, obj: Object) {
        VALUES.lock().unwrap().insert(name, obj);
    }

    pub fn get(&self, token: Token) -> Result<Object, LoxErrors> {
        match VALUES.lock().unwrap().get(&token.lexeme) {
            Some(v) => Ok(v.clone()),
            None => Err(LoxErrors::Environment {
                error_type: EnvironmentErrorType::UnknownVariable,
                msg: format!(
                    "{} -> No such variable '{}'.",
                    token.location(),
                    token.lexeme
                ),
            }),
        }
    }
}
