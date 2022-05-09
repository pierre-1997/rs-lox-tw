use std::collections::HashMap;

use crate::errors::ScannerError;
use crate::token::*;
use crate::token_type::*;

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

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, ScannerError> {
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token()?;
        }

        self.tokens.push(Token::eof(self.line, self.current));

        Ok(&self.tokens)
    }

    fn is_at_end(&self) -> bool {
        self.current == self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), ScannerError> {
        let c = self.advance();
        match c {
            // Single character lexemes
            '(' => self.tokens.push(Token::left_paren(self.line, self.current)),
            ')' => self
                .tokens
                .push(Token::right_paren(self.line, self.current)),
            '{' => self.tokens.push(Token::left_brace(self.line, self.current)),
            '}' => self
                .tokens
                .push(Token::right_brace(self.line, self.current)),
            ',' => self.tokens.push(Token::comma(self.line, self.current)),
            '.' => self.tokens.push(Token::dot(self.line, self.current)),
            '-' => self.tokens.push(Token::minus(self.line, self.current)),
            '+' => self.tokens.push(Token::plus(self.line, self.current)),
            ';' => self.tokens.push(Token::semicolon(self.line, self.current)),
            '*' => self.tokens.push(Token::star(self.line, self.current)),

            // Two character lexemes
            '!' => {
                if self.match_next('=') {
                    self.tokens.push(Token::bang_equal(self.line, self.current));
                } else {
                    self.tokens.push(Token::bang(self.line, self.current));
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.tokens
                        .push(Token::equal_equal(self.line, self.current));
                } else {
                    self.tokens.push(Token::equal(self.line, self.current));
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.tokens.push(Token::less_equal(self.line, self.current));
                } else {
                    self.tokens.push(Token::less(self.line, self.current));
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.tokens
                        .push(Token::greater_equal(self.line, self.current));
                } else {
                    self.tokens.push(Token::greater(self.line, self.current));
                }
            }

            // Special handling of '/' because it can be a comment.
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.tokens.push(Token::slash(self.line, self.current));
                }
            }

            // Meaningless characters
            ' ' => {}
            '\r' => {}
            '\t' => {}

            // Newline
            '\n' => self.line += 1,

            // String literals
            '"' => {
                self.scan_string()?;
            }

            // Unexpected character, throw an error
            _ => {
                // Number literals
                if c.is_digit(10) {
                    self.scan_number()?;
                } else if c.is_alphabetic() || c == '_' {
                    self.scan_identifier();
                } else {
                    return Err(ScannerError::InvalidCharacter);
                }
            }
        }

        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn scan_string(&mut self) -> Result<(), ScannerError> {
        // Keep scanning until we find the closing " or we get to the end of the source code
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        // If we did not find the end of the string, error out
        if self.is_at_end() {
            return Err(ScannerError::UnterminatedString);
        }

        // Read the closing "
        self.advance();

        let token_str = self.source.get(self.start + 1..self.current - 1).unwrap();

        self.tokens
            .push(Token::string(self.line, self.current, token_str));

        Ok(())
    }

    fn scan_number(&mut self) -> Result<(), ScannerError> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        self.tokens.push(Token::number(
            self.line,
            self.start,
            self.current,
            self.source
                .get(self.start..self.current)
                .unwrap()
                .parse::<f64>()
                .ok()
                .unwrap(),
        ));

        Ok(())
    }

    fn scan_identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let substr = self.source.get(self.start..self.current).unwrap();

        let token = match RESERVED_IDENTIFIERS.get(substr) {
            Some(&token_type) => {
                Token::identifier(self.line, self.start, self.current, token_type, substr)
            }
            None => Token::identifier(
                self.line,
                self.start,
                self.current,
                TokenType::IDENTIFIER,
                substr,
            ),
        };

        self.tokens.push(token);
    }
}
