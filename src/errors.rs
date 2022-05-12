use std::fmt;

use crate::token::Token;

#[derive(Debug)]
pub enum RuntimeErrorType {
    UnreachableCode,
    ExpectedNumberOperand,
    ExpectedNumberOperands,
    ExpectedAddableOperands,
}

#[derive(Debug)]
pub enum ScannerErrorType {
    InvalidCharacter,
    UnterminatedString,
}

#[derive(Debug)]
pub enum ParserErrorType {
    ExpectedExpression,
    InvalidConsumeType,
    ExpectedEqual,
}

#[derive(Debug)]
pub enum EnvironmentErrorType {
    UnknownVariable,
}

#[derive(Debug)]
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
        msg: String,
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
                    write!(f, "{} -> {}", token.location(), msg)?
                }
                ParserErrorType::ExpectedExpression => {
                    write!(f, "{} -> {}", token.location(), msg)?
                }
                ParserErrorType::ExpectedEqual => {
                    write!(f, "{} -> Expected equal there.", token.location())?
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

            // Environment errors
            LoxError::EnvironmentError { error_type, msg } => match error_type {
                EnvironmentErrorType::UnknownVariable => write!(f, "{}", msg)?,
            },
        }

        Ok(())
    }
}
