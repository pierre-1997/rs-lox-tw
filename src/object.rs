use std::fmt;
use std::rc::Rc;

use crate::lox_function::LoxFunction;
use crate::lox_native::NativeFunction;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
    Function(Rc<LoxFunction>),
    Native(Rc<NativeFunction>),
}

impl From<bool> for Object {
    fn from(boolean: bool) -> Self {
        match boolean {
            true => Object::True,
            false => Object::False,
        }
    }
}

impl From<f64> for Object {
    fn from(n: f64) -> Self {
        Object::Num(n)
    }
}

impl From<String> for Object {
    fn from(s: String) -> Self {
        Object::Str(s)
    }
}

impl From<&str> for Object {
    fn from(s: &str) -> Self {
        Object::Str(s.to_string())
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(x) => write!(f, "{}", x),
            Self::Str(s) => write!(f, "\"{}\"", s),
            Self::Nil => write!(f, "nil"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Function(fun) => write!(f, "{}", fun),
            Self::Native(fun) => write!(f, "{}", fun),
        }
    }
}
