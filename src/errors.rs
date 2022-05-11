use std::fmt;

pub enum ScannerError {
    InvalidCharacter,
    UnterminatedString,
}

/*
pub fn error(line: usize, msg: String) {
    report(line, "".to_string(), msg);
}

pub fn report(line: usize, location: String, msg: String) {
    eprintln!("[line {}] Error {}: {}", line, location, msg);
}
*/

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScannerError::InvalidCharacter => write!(f, "Invalid character.")?,
            ScannerError::UnterminatedString => write!(f, "Encountered an unterminated string.")?,
        }

        Ok(())
    }
}

pub enum ExprError {}
