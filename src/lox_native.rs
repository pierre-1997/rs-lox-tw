use std::fmt;
use std::rc::Rc;

use crate::lox_callable::LoxCallable;

pub struct NativeFunction {
    pub function: Rc<dyn LoxCallable>,
}

impl PartialEq for NativeFunction {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(
            Rc::as_ptr(&self.function) as *const (),
            Rc::as_ptr(&other.function) as *const (),
        )
    }
}

impl fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native_function>")
    }
}

impl fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native_function>")
    }
}
