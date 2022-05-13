use crate::errors::{LoxErrors, ParserErrorType};
use crate::expr::*;
use crate::stmt::*;
use crate::token::{Object, Token};
use crate::token_type::TokenType;

/**
 * Transforms the given array of tokens into an array of statements.
 */
pub struct Parser<'a> {
    /// The array of tokens to parse.
    tokens: &'a Vec<Token>,
    /// The current index in the array of tokens.
    current: usize,
}

impl<'a> Parser<'a> {
    /**
     * Instanciates a parser from an array of tokens.
     */
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    /**
     * Main parsing function that transforms the array of tokens into an array of statements
     * if they are parsable.
     */
    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxErrors> {
        // Output array of parsed statements
        let mut statements = Vec::new();

        // Parse until reaching the end of the array of tokens
        while !self.is_at_end() {
            // Parse the next tokens into a declaration.
            match self.declaration() {
                Ok(s) => match s {
                    // If the parsed declaration is a statement, save it
                    Some(s) => statements.push(s),
                    None => {}
                },
                // If it is an error, return it
                Err(e) => {
                    return Err(e);
                }
            }
        }

        // Return the parsed statements
        Ok(statements)
    }

    /**
     * Parses the next tokens into a declaration statement.
     */
    fn declaration(&mut self) -> Result<Option<Stmt>, LoxErrors> {
        // If the next token is 'var', parse the variable declaration
        if self.matchs_next(&[TokenType::Var]) {
            match self.var_declaration() {
                // Return the parsed variable declaration statement
                Ok(s) => {
                    return Ok(Some(s));
                }
                // If it was an error, print it and synchronize
                Err(e) => {
                    eprintln!("{e}");
                    self.synchronize();
                }
            }
        }

        // Otherwise, parse it asa statement
        match self.statement() {
            // Return the parsed statement
            Ok(s) => {
                return Ok(Some(s));
            }
            // If it errored, print it and synchronize
            Err(e) => {
                eprintln!("{e}");
                self.synchronize();
            }
        }

        Ok(None)
    }

    /**
     * Parses the next tokens as a variable declaration statement.
     */
    fn var_declaration(&mut self) -> Result<Stmt, LoxErrors> {
        // Expect an indentifier as the variable name.
        let name = self.consume(TokenType::Identifier, "Expected variable name.")?;

        // If we have an '=' after the variable name, means we should then find a value
        let initializer = match self.matchs_next(&[TokenType::Equal]) {
            true => Some(self.expression()?),
            false => None,
        };

        // Check if we got an ending ';' after the variable declaration
        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        )?;

        // Return a non-initialized VarStmt
        Ok(Stmt::Var(VarStmt { name, initializer }))
    }

    /**
     * Parses the next tokens as a statement.
     */
    fn statement(&mut self) -> Result<Stmt, LoxErrors> {
        // Check if the next token is 'print' and if so, parse the print statement
        if self.matchs_next(&[TokenType::Print]) {
            return self.print_statement();
        }

        // Otherwise, parse an expression statement
        self.expression_statement()
    }

    /**
     * Parses the next tokens in a print statement.
     */
    fn print_statement(&mut self) -> Result<Stmt, LoxErrors> {
        // Parse the value to print as an expression
        let value = self.expression()?;
        // Check the statement ends with a semicolon.
        self.consume(TokenType::Semicolon, "Expected ';' after value.")?;
        // Return the parsed print statement
        Ok(Stmt::Print(PrintStmt { expression: value }))
    }

    /**
     * Parses the next tokens in an expression statement.
     */
    fn expression_statement(&mut self) -> Result<Stmt, LoxErrors> {
        // Parse the expression
        let expr = self.expression()?;
        // Check the expression ends with a semicolon.
        self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;
        // Return the parsed expression
        Ok(Stmt::Expression(ExpressionStmt { expression: expr }))
    }

    /**
     * Parse the next tokens as an expression.
     */
    fn expression(&mut self) -> Result<Expr, LoxErrors> {
        // Parse and return the equality
        self.equality()
    }

    /**
     * Parses the next token into an '!=' or '==' expression.
     */
    fn equality(&mut self) -> Result<Expr, LoxErrors> {
        // Parse the comparison
        let mut expr = self.comparison()?;

        // Support of n-member equality expression like a == b == c
        while self.matchs_next(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            // Get the previous token '!=' or '=='
            let operator = self.previous();
            // Get the right part of the expression
            let right = self.comparison()?;
            // Build the binary expression
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        // Return the parsed expression
        Ok(expr)
    }

    /**
     * Parses the nexto tokens into a comparison '>', '>=', '<' or '<=' expression.
     */
    fn comparison(&mut self) -> Result<Expr, LoxErrors> {
        // Get the current terminal expression
        let mut expr = self.term()?;

        // Support of n-member comparison expression like a < b <= c
        while self.matchs_next(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            // Take the previous token as the operator
            let operator = self.previous();
            // Take the next token as the right member of the comparison
            let right = self.term()?;
            // Build the comparison in a binary expression
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        // Return the built expression
        Ok(expr)
    }

    /**
     * Parses the next token into a terminal '-' or '+' expression.
     */
    fn term(&mut self) -> Result<Expr, LoxErrors> {
        // Take the current factor expression
        let mut expr = self.factor()?;

        // Support for n-member terminal expression like a - b + c
        while self.matchs_next(&[TokenType::Minus, TokenType::Plus]) {
            // Take the previous token as the operator
            let operator = self.previous();
            // Take the next token as the right member of the expression
            let right = self.factor()?;
            // Build the terminal expression in a binary one
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        // Return the built expression
        Ok(expr)
    }

    /**
     * Parses the next tokens into a factor '*' or '/' expression.
     */
    fn factor(&mut self) -> Result<Expr, LoxErrors> {
        // Take the next unary expression
        let mut expr = self.unary()?;

        // Support of n-member factor expression like a / b * c
        while self.matchs_next(&[TokenType::Slash, TokenType::Star]) {
            // Take the previous token as the operator
            let operator = self.previous();
            // Take the right member of the expression as an unary expression
            let right = self.unary()?;
            // Build factor expression using binary one
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        // Return the built expression
        Ok(expr)
    }

    /**
     * Parses the next tokens as a unary expression.
     *
     * Note: It can start with '!' or '-', like '-4' or '!true'.
     */
    fn unary(&mut self) -> Result<Expr, LoxErrors> {
        // Check if we are in the case of a '!' or '-' unary expression.
        if self.matchs_next(&[TokenType::Bang, TokenType::Minus]) {
            // Take the previous token as the operator
            let operator = self.previous();
            // Take the next unary expression as the right member of the current unary expression
            // (recursive)
            let right = self.unary()?;
            // Build the unary expression and return it
            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        // Take the next token as a primary expression
        self.primary()
    }

    /**
     * Parses the next single token as a primary expression, meaning a string, number, boolean,
     * Nil or an identifier (example: variable/function name).
     */
    fn primary(&mut self) -> Result<Expr, LoxErrors> {
        // Parse False
        if self.matchs_next(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::False),
            }));
        }

        // Parse True
        if self.matchs_next(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::True),
            }));
        }

        // Parse Nil
        if self.matchs_next(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }

        // Parse a number or a string
        if self.matchs_next(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal,
            }));
        }

        // Parse an identifier
        if self.matchs_next(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(VariableExpr {
                name: self.previous(),
            }));
        }

        // Parse en parenthesized/group expression
        if self.matchs_next(&[TokenType::LeftParen]) {
            // Parse the group enclosed expression
            let expr = self.expression()?;
            // Look for the closing ')' after the grouped expression
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            // Return the built group expression
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }

        // Error out because we expected an expression here
        Err(LoxErrors::Parser {
            token: self.tokens[self.current].dup(),
            error_type: ParserErrorType::ExpectedExpression,
            msg: "".to_string(),
        })
    }

    /**
     * Parses the next token as ttype Token or error out with the given 'msg'
     * string if it isn't one. This function enforces the next token to be of the desired type.
     *
     * Note: This function consumes the token if it is of wanted type.
     */
    fn consume(&mut self, ttype: TokenType, msg: &str) -> Result<Token, LoxErrors> {
        // Check that the next token as the correct type
        if self.check(ttype) {
            return Ok(self.advance());
        }

        // Error out with the given message string
        Err(LoxErrors::Parser {
            token: self.tokens[self.current].dup(),
            error_type: ParserErrorType::InvalidConsumeType,
            msg: msg.to_string(),
        })
    }

    /**
     * Checks tha the next token's type is one of the wanted one.
     * Returns true if it is, false otherwise.
     *
     * Note: This function consumes the token if it is of wanted type.
     */
    fn matchs_next(&mut self, types: &[TokenType]) -> bool {
        // For each of the wanted type, check if the next token if of that type
        for ttype in types {
            // If it is, advance and return true
            if self.check(*ttype) {
                self.advance();
                return true;
            }
        }

        // Return false if the next token does not have the wanted type
        false
    }

    /**
     * Checks if the next token is of the desired 'ttype' type.
     */
    fn check(&self, ttype: TokenType) -> bool {
        // If we are at the end of the token array, return false
        if self.is_at_end() {
            return false;
        }

        // Return the token type comparison result
        self.peek().ttype == ttype
    }

    /**
     * Returns the next token in the array and increment the current index by one.
     */
    fn advance(&mut self) -> Token {
        // If we are not at the end of the array of tokens, increment the currrent index
        if !self.is_at_end() {
            self.current += 1;
        }

        // Return the previous token
        self.previous()
    }

    /**
     * Checks if the current index is at the end of the tokens arra by looking if we are at the Eof
     * token.
     *
     * Returns true if we are, false otherwise.
     */
    fn is_at_end(&self) -> bool {
        // Return the result of the token type comparison with 'Eof'
        self.peek().ttype == TokenType::Eof
    }

    /**
     * Returns a copy of the current token in the array.
     */
    fn peek(&self) -> Token {
        self.tokens[self.current].dup()
    }

    /**
     * Returns a copy of the previous token in the array.
     */
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].dup()
    }

    /**
     * Advances if the tokens until reaching a ';' that would mark the end of the bad code.
     * This function allows for the parser to continue process code even after encountering an
     * error it in.
     */
    fn synchronize(&mut self) {
        // Parse at least one token
        self.advance();

        // We can go up to the end of the whole code if there aren't any way to recover before
        while !self.is_at_end() {
            // If we find a semicolon, we can return
            if self.previous().ttype == TokenType::Semicolon {
                return;
            }

            // Why is that here ?
            match self.peek().ttype {
                TokenType::Class => {}
                TokenType::Fun => {}
                TokenType::Var => {}
                TokenType::For => {}
                TokenType::If => {}
                TokenType::While => {}
                TokenType::Print => {}
                TokenType::Return => {} // TokenType::Class => {}
                _ => {}
            }

            // Advance by one token
            self.advance();
        }
    }
}
