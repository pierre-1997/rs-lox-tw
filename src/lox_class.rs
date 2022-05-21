use std::fmt;

#[derive(Debug)]
pub struct LoxClass {
    pub name: String,
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}
