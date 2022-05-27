use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::errors::{LoxResult, RuntimeErrorType};
use crate::lox_class::LoxClass;
use crate::object::Object;
use crate::token::Token;

/**
 * This structure represents an instanciated class. It contains the class definition itself as well
 * as the declared fields of this specific instance.
 *
 * Note: Fields can actually be methods defined after the class definition.
 */
#[derive(Debug)]
pub struct LoxInstance {
    /// The class this instance comes from.
    pub class: Rc<LoxClass>,
    /// The fields declared for this instance.
    fields: RefCell<HashMap<String, Object>>,
}

impl LoxInstance {
    /**
     * Creates a new instance from a `LoxClass`.
     */
    pub fn new(class: &Rc<LoxClass>) -> Self {
        LoxInstance {
            class: Rc::clone(class),
            fields: RefCell::new(HashMap::new()),
        }
    }

    /**
     * This function is used to retrieve any field associated with this instance. It is used when
     * calling `instance.X`, with 'X' being the field/method we want to retrieve from this class
     * instance.
     *
     * Note: This function needs to specially handle the `this` keyword.
     */
    pub fn get(&self, name: &Token, this: &Object) -> Result<Object, LoxResult> {
        println!("Getting {} from {:?}", name.lexeme, self.fields);
        // Look for a field with that name
        if let Some(field) = self.fields.borrow_mut().get(&name.lexeme) {
            Ok(field.clone())
        }
        // Look for a method with that name
        else if let Some(method) = self.class.find_method(&name.lexeme) {
            Ok(Object::Function(Rc::new(method.bind(this.clone()))))
        } else {
            Err(LoxResult::Runtime {
                token: name.clone(),
                error_type: RuntimeErrorType::UndefinedProperty,
            })
        }
    }

    pub fn set(&self, name: &Token, value: Object) {
        self.fields.borrow_mut().insert(name.lexeme.clone(), value);
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<instance of {}", self.class)?;

        if !self.fields.borrow().is_empty() {
            for (name, obj) in self.fields.borrow().iter() {
                writeln!(f, "- this.{} = {}", name, obj)?;
            }
        } else {
            writeln!(f, "No defined properties.")?;
        }

        writeln!(f, ">")?;

        Ok(())
    }
}
