use std::fmt;
use std::hash::Hash;

use crate::object::Object;
use crate::token_type::*;

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub src_line: usize,
    pub src_start: usize,
    pub src_end: usize,
}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ttype.hash(state);
        self.lexeme.hash(state);
    }
}

// TODO: Should we compare token position too ?
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.ttype == other.ttype && self.lexeme == other.lexeme && self.literal == other.literal
    }
}

impl Eq for Token {}

impl Token {
    pub fn location(&self) -> String {
        format!(
            "Line {} [{}:{}]",
            self.src_line, self.src_start, self.src_end
        )
    }

    pub fn eof(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn left_paren(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::LeftParen,
            lexeme: "(".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn right_paren(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::RightParen,
            lexeme: ")".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn left_brace(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::LeftBrace,
            lexeme: "{".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn right_brace(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::RightBrace,
            lexeme: "}".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn comma(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::Comma,
            lexeme: ",".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn dot(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::Dot,
            lexeme: ".".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn minus(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::Minus,
            lexeme: "-".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn plus(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::Plus,
            lexeme: "+".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn semicolon(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::Semicolon,
            lexeme: ";".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn star(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::Star,
            lexeme: "*".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn bang(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::Bang,
            lexeme: "!".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn bang_equal(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::BangEqual,
            lexeme: "!=".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 2,
        }
    }

    pub fn equal(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::Equal,
            lexeme: "=".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn equal_equal(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::EqualEqual,
            lexeme: "==".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 2,
        }
    }

    pub fn less(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::Less,
            lexeme: "<".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn less_equal(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::LessEqual,
            lexeme: "<=".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 2,
        }
    }

    pub fn greater(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::Greater,
            lexeme: ">".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn greater_equal(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::GreaterEqual,
            lexeme: ">=".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 2,
        }
    }

    pub fn slash(src_line: usize, src_at: usize) -> Token {
        Token {
            ttype: TokenType::Slash,
            lexeme: "/".to_string(),
            literal: None,
            src_line,
            src_start: src_at,
            src_end: src_at + 1,
        }
    }

    pub fn string(src_line: usize, src_at: usize, s: &str) -> Token {
        Token {
            ttype: TokenType::String,
            lexeme: "".to_string(),
            literal: Some(Object::Str(s.to_string())),
            src_line,
            src_start: src_at,
            src_end: src_at + s.len(),
        }
    }

    pub fn number(src_line: usize, src_start: usize, src_end: usize, n: f64) -> Token {
        Token {
            ttype: TokenType::Number,
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
