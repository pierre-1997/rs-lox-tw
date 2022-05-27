use std::fmt;
use std::rc::Rc;

use crate::lox_class::LoxClass;
use crate::lox_function::LoxFunction;
use crate::lox_instance::LoxInstance;
use crate::lox_native::NativeFunction;

#[derive(Debug, Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
    Function(Rc<LoxFunction>),
    Native(Rc<NativeFunction>),
    Class(Rc<LoxClass>),
    Instance(Rc<LoxInstance>),
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Num(a), Object::Num(b)) => a == b,
            (Object::Str(a), Object::Str(b)) => a == b,
            (Object::True, Object::True) => true,
            (Object::False, Object::False) => true,
            (Object::Nil, Object::Nil) => true,

            _ => false,
        }
    }
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
            Self::Num(x) => write!(f, "{x}"),
            Self::Str(s) => write!(f, "\"{s}\""),
            Self::Nil => write!(f, "nil"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Function(fun) => write!(f, "{fun}"),
            Self::Native(fun) => write!(f, "{fun}"),
            Self::Class(class) => write!(f, "{class}"),
            Self::Instance(instance) => write!(f, "{instance}"),
        }
    }
}
