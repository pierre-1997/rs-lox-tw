use std::fmt;

use crate::object::Object;
use crate::token::Token;

#[derive(Debug)]
pub enum RuntimeErrorType {
    UnreachableCode,
    ExpectedNumberOperand,
    Panic,
    ExpectedNumberOperands,
    ExpectedAddableOperands,
    InvalidCallObjectType,
    InvalidArgsCount,
    InvalidObjectProperty,
    UndefinedProperty,
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
    InvalidAssignTarget,
    MaxArgNumber,
}

#[derive(Debug)]
pub enum ResolverErrorType {
    VariableNotInitialized,
    VariableAlreadyExists,
    TopLevelReturn,
    Panic,
}

#[derive(Debug)]
pub enum EnvironmentErrorType {
    UnknownVariable,
}

#[derive(Debug)]
pub enum LoxResult {
    Parser {
        token: Token,
        error_type: ParserErrorType,
        msg: String,
    },
    Runtime {
        token: Token,
        error_type: RuntimeErrorType,
    },
    Scanner {
        c: char,
        error_type: ScannerErrorType,
    },
    Environment {
        error_type: EnvironmentErrorType,
        msg: String,
    },
    ReturnValue {
        value: Object,
    },
    Resolver {
        token: Token,
        error_type: ResolverErrorType,
    },
}

impl LoxResult {
    /*
    pub fn error() -> Self{
        report(line, "".to_string(), msg);
    }

    pub fn report(line: usize, location: String, msg: String) -> Self{
        eprintln!("[line {}] Error {}: {}", line, location, msg);
    }
    */
}

impl fmt::Display for LoxResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxResult::Scanner { c, error_type } => match error_type {
                ScannerErrorType::InvalidCharacter => write!(f, "Invalid character {c}.")?,
                ScannerErrorType::UnterminatedString => {
                    write!(f, "Encountered an unterminated string.")?
                }
            },

            // Parser error
            LoxResult::Parser {
                token,
                error_type,
                msg,
            } => match error_type {
                ParserErrorType::InvalidConsumeType => write!(f, "{} -> {msg}", token.location())?,
                ParserErrorType::ExpectedExpression => write!(f, "{} -> {msg}", token.location())?,
                ParserErrorType::InvalidAssignTarget => {
                    write!(f, "{} -> Invalid assignment target.", token.location())?
                }
                ParserErrorType::MaxArgNumber => write!(
                    f,
                    "{} -> Cannot have more than 255 arguments.",
                    token.location()
                )?,
            },

            // Runtime error
            LoxResult::Runtime { token, error_type } => match error_type {
                RuntimeErrorType::Panic => write!(f, "{} -> WTFDFFFFF !!!", token.location())?,
                RuntimeErrorType::UnreachableCode => {
                    writeln!(f, "{} -> This code is unreachable.", token.location())?;
                }
                RuntimeErrorType::ExpectedNumberOperand => {
                    write!(f, "{} -> Operand must be a number.", token.location())?
                }
                RuntimeErrorType::ExpectedNumberOperands => {
                    write!(f, "{} -> Both operands must be a number.", token.location())?
                }
                RuntimeErrorType::InvalidCallObjectType => write!(
                    f,
                    "{} -> Can only call functions and classes.",
                    token.location()
                )?,
                RuntimeErrorType::ExpectedAddableOperands => write!(
                    f,
                    "{} -> Operands must be two numbers or two strings.",
                    token.location()
                )?,
                RuntimeErrorType::InvalidArgsCount => write!(
                    f,
                    "{} -> Invalid argument count for {} or class.",
                    token.location(),
                    token.lexeme
                )?,
                RuntimeErrorType::InvalidObjectProperty => {
                    write!(f, "{} -> Only classes have properties.", token.location())?
                }
                RuntimeErrorType::UndefinedProperty => write!(
                    f,
                    "{} -> Undefined property {} for this class.",
                    token.location(),
                    token.lexeme
                )?,
            },

            // Environment errors
            LoxResult::Environment { error_type, msg } => match error_type {
                EnvironmentErrorType::UnknownVariable => write!(f, "{msg}")?,
            },

            // Return value
            LoxResult::ReturnValue { value } => write!(f, "return {value}")?,

            // Resolver Error
            LoxResult::Resolver { token, error_type } => match error_type {
                ResolverErrorType::Panic => write!(f, "{} -> WTFFFF !!!", token.location())?,
                ResolverErrorType::VariableNotInitialized => write!(
                    f,
                    "{} -> Can't read local variable in its own initializer.",
                    token.location()
                )?,
                ResolverErrorType::VariableAlreadyExists => write!(
                    f,
                    "{} -> A variable with the name '{}' already exists in this scope.",
                    token.location(),
                    token.lexeme
                )?,
                ResolverErrorType::TopLevelReturn => write!(
                    f,
                    "{} -> Can't return from top level code.",
                    token.location()
                )?,
            },
        }

        Ok(())
    }
}
