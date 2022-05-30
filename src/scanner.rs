use std::collections::HashMap;

use crate::errors::{LoxResult, ScannerErrorType};
use crate::token::*;
use crate::token_type::*;

use lazy_static::lazy_static;
lazy_static! {
    /// An `HashMap` containing the reserved words of the lox language.
    static ref RESERVED_IDENTIFIERS: HashMap<String, TokenType> = HashMap::from([
        ("and".to_string(), TokenType::And),
        ("class".to_string(), TokenType::Class),
        ("else".to_string(), TokenType::Else),
        ("false".to_string(), TokenType::False),
        ("for".to_string(), TokenType::For),
        ("fun".to_string(), TokenType::Fun),
        ("if".to_string(), TokenType::If),
        ("nil".to_string(), TokenType::Nil),
        ("or".to_string(), TokenType::Or),
        ("print".to_string(), TokenType::Print),
        ("return".to_string(), TokenType::Return),
        ("super".to_string(), TokenType::Super),
        ("this".to_string(), TokenType::This),
        ("true".to_string(), TokenType::True),
        ("var".to_string(), TokenType::Var),
        ("while".to_string(), TokenType::While),
    ]);
}

/**
 * The Scanner object
 */
pub struct Scanner {
    /// The raw source code as a String.
    pub source: String,
    /// The vector of `Token` parsed.
    pub tokens: Vec<Token>,
    /// The start of the current token (index in `self.source`).
    start: usize,
    /// The index in `self.source` the scanner is currently at.
    current: usize,
    /// The current line number being scanned.
    line: usize,
}

impl Scanner {
    /**
     * Instanciates a new `Scanner` from raw source code as a String.
     */
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            // Source code is written from line 1
            line: 1,
        }
    }

    /**
     * Scanner's main function that will run through the source code and turn it into a vector of
     * `Token` structs.
     */
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxResult> {
        // Scan a token at a time until reaching the end of the source code.
        while !self.is_at_end() {
            // Reset the start of the token to the current position
            self.start = self.current;
            // Scan a token
            self.scan_token()?;
        }

        // Append a terminal `Eof` token at the end of the source code.
        self.tokens.push(Token::eof(self.line, self.current));

        // Return the parsed tokens
        Ok(&self.tokens)
    }

    /**
     * Helper that returns true if we reached the end of the source code.
     */
    fn is_at_end(&self) -> bool {
        // Simply check the current position with the size of the source code
        self.current == self.source.len()
    }

    /**
     * Scan a single token from the source code. Appends it to `self.tokens`.
     */
    fn scan_token(&mut self) -> Result<(), LoxResult> {
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
                    return Err(LoxResult::Scanner {
                        c,
                        error_type: ScannerErrorType::InvalidCharacter,
                    });
                }
            }
        }

        Ok(())
    }

    /**
     * Returns the next source code's character.
     *
     * Note: Increments `self.current`.
     */
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    /**
     * Checks the next charater of the source code with an expected one. Returns true if it
     * matches, false otherwise
     *
     * Note: If the next character matches, it will get consumed (e.g. `self.current` will get
     * incremented).
     */
    fn match_next(&mut self, expected: char) -> bool {
        // Return false if we're at the end of the source code
        if self.is_at_end() {
            return false;
        }

        // If it is different, return false
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        // It matched, increment `self.current` and return true
        self.current += 1;
        true
    }

    /**
     * Helper that returns the current character without consuming it
     * (e.g. without incrementing `self.current`).
     */
    fn peek(&self) -> char {
        // Return `\0` if we reached the end of the source code
        if self.is_at_end() {
            return '\0';
        }

        // Return the current character
        return self.source.chars().nth(self.current).unwrap();
    }

    /**
     * Helper that returns the next character without consuming it
     * (e.g. without incrementing `self.current`).
     */
    fn peek_next(&self) -> char {
        // Check if the next char before is the end of file
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        // Return the next character
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    /**
     * Called when a string has been detected in the source code. This function will
     * parse the next tokens as a `Token::string` variant.
     *
     * Note: This function will apend the parsed `Token` into `self.tokens`.
     */
    fn scan_string(&mut self) -> Result<(), LoxResult> {
        // Keep scanning until we find the closing `"` or we get to the end of the
        // source code
        while self.peek() != '"' && !self.is_at_end() {
            // Don't forget to increment `self.line` on newline
            if self.peek() == '\n' {
                self.line += 1;
            }
            // Advance by one char
            self.advance();
        }

        // If we did not find the end of the string, error out
        if self.is_at_end() {
            return Err(LoxResult::Scanner {
                c: '"',
                error_type: ScannerErrorType::UnterminatedString,
            });
        }

        // Read the closing `"`
        self.advance();
        // Get a substring of the source code using `self.start` and `self.current`
        let token_str = self.source.get(self.start + 1..self.current - 1).unwrap();
        // Push the parsed `Token::string` in `self.tokens`
        self.tokens
            .push(Token::string(self.line, self.current, token_str));

        Ok(())
    }

    /**
     * Called when a number has been detected in the source code. This function will
     * parse the next tokens as a `Token::number` variant.
     *
     * Note: This function will apend the parsed `Token` into `self.tokens`.
     */
    fn scan_number(&mut self) -> Result<(), LoxResult> {
        // Advance as long as we find numbers
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Check if we stopped by a `.` followed by another number
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // If so, advance after the `.`
            self.advance();
            // And advance as long as we find numbers
            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        // Parse the substring of the source code containing the number into a `f64`
        // and then into a `Token::number` variant and push it in `self.tokens`.
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

    /**
     * Called when an identifier (e.g. a variable/function/class name) has been detected
     * in the source code. This function will parse the next tokens as a
     * `Token::identifier` variant.
     *
     * Note: This function will apend the parsed `Token` into `self.tokens`.
     */
    fn scan_identifier(&mut self) {
        // Advance as long as we're finding alphanumerical or `_`
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        // Get the substring of the source code that contains the identifier
        let substr = self.source.get(self.start..self.current).unwrap();
        // Check if it is a reserved lox identifier (ex: for, if, else, etc)
        let token = match RESERVED_IDENTIFIERS.get(substr) {
            Some(&token_type) => {
                Token::identifier(self.line, self.start, self.current, token_type, substr)
            }
            // Else, return an `Token::identifier` variant with `TokenType::Identifier`
            None => Token::identifier(
                self.line,
                self.start,
                self.current,
                TokenType::Identifier,
                substr,
            ),
        };

        // Append the parsed token to `self.tokens`
        self.tokens.push(token);
    }
}
