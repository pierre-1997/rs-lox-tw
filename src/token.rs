use crate::token_type::*;
use std::{collections::HashMap, fmt};

use lazy_static::lazy_static;
lazy_static! {
    static ref RESERVED_IDENTIFIERS: HashMap<String, TokenType> = HashMap::from([
        ("and".to_string(), TokenType::AND),
        ("class".to_string(), TokenType::CLASS),
        ("else".to_string(), TokenType::ELSE),
        ("false".to_string(), TokenType::FALSE),
        ("for".to_string(), TokenType::FOR),
        ("fun".to_string(), TokenType::FUN),
        ("if".to_string(), TokenType::IF),
        ("nil".to_string(), TokenType::NIL),
        ("or".to_string(), TokenType::OR),
        ("print".to_string(), TokenType::PRINT),
        ("return".to_string(), TokenType::RETURN),
        ("super".to_string(), TokenType::SUPER),
        ("this".to_string(), TokenType::THIS),
        ("true".to_string(), TokenType::TRUE),
        ("var".to_string(), TokenType::VAR),
        ("while".to_string(), TokenType::WHILE),
    ]);
}

#[derive(Debug, PartialEq)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Num(x) => write!(f, "{}", x),
            Self::Str(s) => write!(f, "\"{}\"", s),
            Self::Nil => write!(f, "nil"),
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
        }
    }
}

#[derive(Debug)]
pub struct Token {
    ttype: TokenType,
    pub lexeme: String,
    literal: Option<Object>,
    src_line: usize,
    src_start: usize,
    src_end: usize,
}

impl Token {
    pub fn eof(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn left_paren(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::LEFT_PAREN,
            lexeme: "(".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn right_paren(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::RIGHT_PAREN,
            lexeme: ")".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn left_brace(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::LEFT_BRACE,
            lexeme: "{".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn right_brace(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::RIGHT_BRACE,
            lexeme: "}".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn comma(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::COMMA,
            lexeme: ",".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn dot(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::DOT,
            lexeme: ".".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn minus(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::MINUS,
            lexeme: "-".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn plus(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::PLUS,
            lexeme: "+".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn semicolon(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::SEMICOLON,
            lexeme: ";".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn star(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::STAR,
            lexeme: "*".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn bang(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::BANG,
            lexeme: "!".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn bang_equal(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::BANG_EQUAL,
            lexeme: "!=".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 2,
        }
    }

    pub fn equal(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::EQUAL,
            lexeme: "=".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn equal_equal(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::EQUAL_EQUAL,
            lexeme: "==".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 2,
        }
    }

    pub fn less(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::LESS,
            lexeme: "<".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn less_equal(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::LESS_EQUAL,
            lexeme: "<=".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 2,
        }
    }

    pub fn greater(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::GREATER,
            lexeme: ">".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn greater_equal(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::GREATER_EQUAL,
            lexeme: ">=".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 2,
        }
    }

    pub fn slash(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::SLASH,
            lexeme: "/".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn string(src_line: usize, src_at: usize, s: &str) -> Token {
        Token {
            ttype: TokenType::STRING,
            lexeme: "".to_string(),
            literal: Some(Object::Str(s.to_string())),
            src_line,
            src_start: src_at,
            src_end: src_at + s.len(),
        }
    }

    pub fn number(src_line: usize, src_start: usize, src_end: usize, n: f64) -> Token {
        Token {
            ttype: TokenType::NUMBER,
            lexeme: "".to_string(),
            literal: Some(Object::Num(n)),
            src_line,
            src_start,
            src_end,
        }
    }

    pub fn identifier(
        src_line: usize,
        src_start: usize,
        src_end: usize,
        ttype: TokenType,
        l: &str,
    ) -> Token {
        Token {
            ttype,
            lexeme: l.to_string(),
            literal: None,
            src_line,
            src_start,
            src_end,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{type: {:?}, lexeme: {}, literal: {}, line: {}}}",
            self.ttype,
            self.lexeme,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            },
            self.src_line
        )
    }
}
