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

pub enum ExprError {
    UnreachableCode,
    InvalidExpression,
    ExpectedNumberOperand,
    ExpectedNumberOperands,
    ExpectedAddableOperands,
}

impl From<EnvironmentError> for ExprError {
    fn from(ee: EnvironmentError) -> Self {
        match ee {
            EnvironmentError::UnknownVariable => Self::InvalidExpression,
        }
    }
}

impl From<StmtError> for ExprError {
    fn from(_: StmtError) -> Self {
        Self::InvalidExpression
    }
}

impl fmt::Display for ExprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExprError::UnreachableCode => write!(f, "Unreachable code."),
            ExprError::InvalidExpression => write!(f, "Invalid expression."),
            ExprError::ExpectedNumberOperand => write!(f, "Operand must be a number."),
            ExprError::ExpectedNumberOperands => write!(f, "Both operands must be a number."),

            ExprError::ExpectedAddableOperands => {
                write!(f, "Operands must be two numbers or two strings.")
            }
        }
    }
}

pub enum ParserError {
    ExpectedExpression,
    InvalidConsumeType,
    ExpectedEqual,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidConsumeType => write!(f, "consume() invalid type"),
            ParserError::ExpectedExpression => write!(f, "Expected expression."),
            ParserError::ExpectedEqual => write!(f, "Expected equal there."),
        }
    }
}

pub enum RunError {}

#[derive(Debug)]
pub enum StmtError {
    GenericStmtError,
}

impl From<ExprError> for StmtError {
    fn from(_: ExprError) -> Self {
        Self::GenericStmtError
    }
}

pub enum EnvironmentError {
    UnknownVariable,
}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvironmentError::UnknownVariable => write!(f, "Undefined variable."),
        }
    }
}
