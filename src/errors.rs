use std::fmt;

use crate::object::Object;
use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum RuntimeErrorType {
    UnreachableCode,
    ExpectedNumberOperand,
    ExpectedNumberOperands,
    ExpectedAddableOperands,
    InvalidCallObjectType,
    InvalidArgsCount,
    InvalidObjectProperty,
    UndefinedProperty,
}

#[derive(Debug, PartialEq)]
pub enum ScannerErrorType {
    InvalidCharacter,
    UnterminatedString,
}

#[derive(Debug, PartialEq)]
pub enum ParserErrorType {
    ExpectedExpression,
    InvalidConsumeType,
    InvalidAssignTarget,
    MaxArgNumber,
}

#[derive(Debug, PartialEq)]
pub enum ResolverErrorType {
    VariableNotInitialized,
    VariableAlreadyExists,
    TopLevelReturn,
}

#[derive(Debug, PartialEq)]
pub enum EnvironmentErrorType {
    UnknownVariable,
}

#[derive(Debug, PartialEq)]
pub enum LoxResult {
    IOError,
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

impl fmt::Display for LoxResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // IO reading error
            LoxResult::IOError => write!(f, "[IOError] There was an IO error.")?,

            // Scanner error
            LoxResult::Scanner { c, error_type } => match error_type {
                ScannerErrorType::InvalidCharacter => {
                    write!(f, "[scanner] Invalid character {c}.")?
                }
                ScannerErrorType::UnterminatedString => {
                    write!(f, "[scanner] Encountered an unterminated string.")?
                }
            },

            // Parser error
            LoxResult::Parser {
                token,
                error_type,
                msg,
            } => match error_type {
                ParserErrorType::InvalidConsumeType => {
                    write!(f, "[parser] {} -> {msg}", token.location())?
                }
                ParserErrorType::ExpectedExpression => {
                    write!(f, "[parser] {} -> {msg}", token.location())?
                }
                ParserErrorType::InvalidAssignTarget => write!(
                    f,
                    "[parser] {} -> Invalid assignment target.",
                    token.location()
                )?,
                ParserErrorType::MaxArgNumber => write!(
                    f,
                    "[parser] {} -> Cannot have more than 255 arguments.",
                    token.location()
                )?,
            },

            // Runtime error
            LoxResult::Runtime { token, error_type } => match error_type {
                RuntimeErrorType::UnreachableCode => {
                    writeln!(
                        f,
                        "[runtime] {} -> This code is unreachable.",
                        token.location()
                    )?;
                }
                RuntimeErrorType::ExpectedNumberOperand => write!(
                    f,
                    "[runtime] {} -> Operand must be a number.",
                    token.location()
                )?,
                RuntimeErrorType::ExpectedNumberOperands => write!(
                    f,
                    "[runtime] {} -> Both operands must be a number.",
                    token.location()
                )?,
                RuntimeErrorType::InvalidCallObjectType => write!(
                    f,
                    "[runtime] {} -> Can only call functions and classes.",
                    token.location()
                )?,
                RuntimeErrorType::ExpectedAddableOperands => write!(
                    f,
                    "[runtime] {} -> Operands must be two numbers or two strings.",
                    token.location()
                )?,
                RuntimeErrorType::InvalidArgsCount => write!(
                    f,
                    "[runtime] {} -> Invalid argument count for {} or class.",
                    token.location(),
                    token.lexeme
                )?,
                RuntimeErrorType::InvalidObjectProperty => write!(
                    f,
                    "[runtime] {} -> Only classes have properties.",
                    token.location()
                )?,
                RuntimeErrorType::UndefinedProperty => write!(
                    f,
                    "[runtime] {} -> Undefined property {} for this class.",
                    token.location(),
                    token.lexeme
                )?,
            },

            // Environment errors
            LoxResult::Environment { error_type, msg } => match error_type {
                EnvironmentErrorType::UnknownVariable => write!(f, "[env] {msg}")?,
            },

            // Return value
            LoxResult::ReturnValue { value } => write!(f, "return {value}")?,

            // Resolver Error
            LoxResult::Resolver { token, error_type } => match error_type {
                ResolverErrorType::VariableNotInitialized => write!(
                    f,
                    "[resolver] {} -> Can't read local variable in its own initializer.",
                    token.location()
                )?,
                ResolverErrorType::VariableAlreadyExists => write!(
                    f,
                    "[resolver] {} -> A variable with the name '{}' already exists in this scope.",
                    token.location(),
                    token.lexeme
                )?,
                ResolverErrorType::TopLevelReturn => write!(
                    f,
                    "[resolver] {} -> Can't return from top level code.",
                    token.location()
                )?,
            },
        }

        Ok(())
    }
}
