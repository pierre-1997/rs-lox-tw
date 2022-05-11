use std::fmt;

use crate::token::Token;

pub enum RuntimeErrorType {
    UnreachableCode,
    ExpectedNumberOperand,
    ExpectedNumberOperands,
    ExpectedAddableOperands,
}
pub enum ScannerErrorType {
    InvalidCharacter,
    UnterminatedString,
}

pub enum ParserErrorType {
    ExpectedExpression,
    InvalidConsumeType,
    ExpectedEqual,
}

pub enum EnvironmentErrorType {
    UnknownVariable,
}

pub enum LoxError {
    ParserError {
        token: Token,
        error_type: ParserErrorType,
        msg: String,
    },
    RuntimeError {
        error_type: RuntimeErrorType,
    },
    ScannerError {
        c: char,
        error_type: ScannerErrorType,
    },
    EnvironmentError {
        error_type: EnvironmentErrorType,
    },
}

impl LoxError {
    /*
    pub fn error() -> Self{
        report(line, "".to_string(), msg);
    }

    pub fn report(line: usize, location: String, msg: String) -> Self{
        eprintln!("[line {}] Error {}: {}", line, location, msg);
    }
    */
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::ScannerError { c, error_type } => match error_type {
                ScannerErrorType::InvalidCharacter => write!(f, "Invalid character {c}.")?,
                ScannerErrorType::UnterminatedString => {
                    write!(f, "Encountered an unterminated string.")?
                }
            },

            // Parser error
            LoxError::ParserError {
                token,
                error_type,
                msg,
            } => match error_type {
                ParserErrorType::InvalidConsumeType => {
                    writeln!(f, "Error at token [{}]", token)?;
                    writeln!(f, "{}", msg)?;
                }
                ParserErrorType::ExpectedExpression => {
                    writeln!(f, "Error at token [{}]", token)?;
                    writeln!(f, "Expected expression.")?;
                }
                ParserErrorType::ExpectedEqual => {
                    writeln!(f, "Error at token [{}]", token)?;
                    write!(f, "Expected equal there.")?;
                }
            },

            // Runtime error
            LoxError::RuntimeError { error_type } => match error_type {
                RuntimeErrorType::UnreachableCode => {
                    writeln!(f, "This code is unreachable.")?;
                }
                RuntimeErrorType::ExpectedNumberOperand => write!(f, "Operand must be a number.")?,
                RuntimeErrorType::ExpectedNumberOperands => {
                    write!(f, "Both operands must be a number.")?
                }

                RuntimeErrorType::ExpectedAddableOperands => {
                    write!(f, "Operands must be two numbers or two strings.")?
                }
            },

            _ => write!(f, "Unhandled error yet.")?,
        }

        Ok(())
    }
}
