use std::borrow::Borrow;
use std::fmt;
use std::rc::Rc;

use crate::lox_class::LoxClass;

#[derive(Debug)]
pub struct LoxInstance {
    class: Rc<LoxClass>,
}

impl LoxInstance {
    pub fn new(class: &Rc<LoxClass>) -> Self {
        LoxInstance {
            class: Rc::clone(class),
        }
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<instance of {}>", self.class)
    }
}
