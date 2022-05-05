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

pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn eof(line: usize) -> Token {
        Token {
            ttype: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line,
        }
    }

    pub fn left_paren() -> Token {
        Token {
            ttype: TokenType::LEFT_PAREN,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn right_paren() -> Token {
        Token {
            ttype: TokenType::RIGHT_PAREN,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn left_brace() -> Token {
        Token {
            ttype: TokenType::LEFT_BRACE,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn right_brace() -> Token {
        Token {
            ttype: TokenType::RIGHT_BRACE,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn comma() -> Token {
        Token {
            ttype: TokenType::COMMA,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn dot() -> Token {
        Token {
            ttype: TokenType::DOT,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn minus() -> Token {
        Token {
            ttype: TokenType::MINUS,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn plus() -> Token {
        Token {
            ttype: TokenType::PLUS,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn semicolon() -> Token {
        Token {
            ttype: TokenType::SEMICOLON,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn star() -> Token {
        Token {
            ttype: TokenType::STAR,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn bang() -> Token {
        Token {
            ttype: TokenType::BANG,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn bang_equal() -> Token {
        Token {
            ttype: TokenType::BANG_EQUAL,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn equal() -> Token {
        Token {
            ttype: TokenType::EQUAL,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn equal_equal() -> Token {
        Token {
            ttype: TokenType::EQUAL_EQUAL,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn less() -> Token {
        Token {
            ttype: TokenType::LESS,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn less_equal() -> Token {
        Token {
            ttype: TokenType::LESS_EQUAL,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn greater() -> Token {
        Token {
            ttype: TokenType::GREATER,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn greater_equal() -> Token {
        Token {
            ttype: TokenType::GREATER_EQUAL,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn slash() -> Token {
        Token {
            ttype: TokenType::SLASH,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn string(s: String) -> Token {
        Token {
            ttype: TokenType::STRING,
            lexeme: "".to_string(),
            literal: Some(Object::Str(s)),
            line: 0,
        }
    }

    pub fn number(n: f64) -> Token {
        Token {
            ttype: TokenType::NUMBER,
            lexeme: "".to_string(),
            literal: Some(Object::Num(n)),
            line: 0,
        }
    }

    pub fn or() -> Token {
        Token {
            ttype: TokenType::OR,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn identifier() -> Token {
        Token {
            ttype: TokenType::IDENTIFIER,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }

    pub fn from_type(tt: TokenType) -> Token {
        Token {
            ttype: tt,
            lexeme: "".to_string(),
            literal: None,
            line: 0,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Type: {:?}", self.ttype)
    }
}
