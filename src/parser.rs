use crate::errors::{LoxError, ParserErrorType};
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
    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
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
    fn declaration(&mut self) -> Result<Option<Stmt>, LoxError> {
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
    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
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
    fn statement(&mut self) -> Result<Stmt, LoxError> {
        // Check if the next token is the start of a for-statement\
        if self.matchs_next(&[TokenType::For]) {
            return self.for_statement();
        }

        // Check if the next token is the start of an if-statement
        if self.matchs_next(&[TokenType::If]) {
            return self.if_statement();
        }

        // Check if the next token is 'print' and if so, parse the print statement
        if self.matchs_next(&[TokenType::Print]) {
            return self.print_statement();
        }

        // Check if the next statement is a 'while' loop
        if self.matchs_next(&[TokenType::While]) {
            return self.while_statement();
        }

        // Check if the next token is a scope opening left brace '{'
        if self.matchs_next(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(BlockStmt {
                statements: self.block_statement()?,
            }));
        }

        // Otherwise, parse an expression statement
        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<Stmt, LoxError> {
        // The next token to come after 'for' must be an opening '('
        self.consume(
            TokenType::LeftParen,
            "Expected opening '(' after 'for' statement.",
        )?;

        // Parsing the initializer if any
        let initializer;
        // for (; ...) -> no initializer
        if self.matchs_next(&[TokenType::Semicolon]) {
            initializer = None;
        // for (var ..; ...) -> initializer is a var declaration
        } else if self.matchs_next(&[TokenType::Var]) {
            initializer = Some(self.var_declaration()?);
        // for (<expression>; ...)
        } else {
            initializer = Some(self.expression_statement()?);
        }

        // Parsing the condition if any
        let mut condition = None;
        // for(<initializer>; <condition> ; ...)
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        // Check that the condition is correctly followed by a ';'
        self.consume(TokenType::Semicolon, "Expected ';' after loop condition.")?;

        // Parsing the increment if any
        let mut increment = None;
        // If we are not at the closing ')', parse the increment
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }

        // Check that the for statement is correctly ending with a ')'
        self.consume(
            TokenType::RightParen,
            "Expected closing ')' after for statement.",
        )?;

        /*
         * We will now basically transform the for loop into a while loop here.
         *
         * Written for loop:
         * for (var i = 0; i < 10; i = i + 1) print i;
         *
         * Executed while loop:
         * {
         * var i = 0;
         * while (i < 10) {
         *  print i;
         *  i = i + 1;
         * }
        }
        */

        // Parse the body statements of the for loop
        // e.g in the example above: "print i;"
        let mut body = self.statement()?;

        // If there were an increment, write an iteration of it at the end of the body.
        // e.g in the example above: "i = i + 1"
        if let Some(i) = increment {
            body = Stmt::Block(BlockStmt {
                statements: vec![body, Stmt::Expression(ExpressionStmt { expression: i })],
            })
        }

        // If there weren't any condition, write a true literal expression instead to a perform a
        // while (true) infinite loop.
        if condition.is_none() {
            condition = Some(Expr::Literal(LiteralExpr {
                value: Some(Object::True),
            }));
        }

        // Put the current body into a while expression with its condition
        body = Stmt::While(WhileStmt {
            condition: condition.unwrap(),
            body: Box::new(body),
        });

        // If there were any initializer, put it at the beggining of the new tranformed code
        // e.g in the example above: "var i = 0;"
        if initializer.is_some() {
            body = Stmt::Block(BlockStmt {
                statements: vec![initializer.unwrap(), body],
            });
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if' statement.")?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            "Missing closing ')' after if condition.",
        )?;

        let then_branch = self.statement()?;
        let mut else_branch = None;

        if self.matchs_next(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Stmt::If(IfStmt {
            condition,
            then_branch: Box::new(then_branch),
            else_branch,
        }))
    }

    /**
     * Parses the next tokens in a print statement.
     */
    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        // Parse the value to print as an expression
        let value = self.expression()?;
        // Check the statement ends with a semicolon.
        self.consume(TokenType::Semicolon, "Expected ';' after value.")?;
        // Return the parsed print statement
        Ok(Stmt::Print(PrintStmt { expression: value }))
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expected '(' after while statement.")?;
        let condition = self.expression()?;
        self.consume(
            TokenType::LeftParen,
            "Expected closing ')' after while statement.",
        )?;
        let body = self.statement()?;

        Ok(Stmt::While(WhileStmt {
            condition,
            body: Box::new(body),
        }))
    }

    fn block_statement(&mut self) -> Result<Vec<Stmt>, LoxError> {
        let mut stmts = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            if let Some(s) = self.declaration()? {
                stmts.push(s);
            }
        }

        self.consume(
            TokenType::RightBrace,
            "Missing scope ending right brace '}'.",
        )?;

        Ok(stmts)
    }

    /**
     * Parses the next tokens in an expression statement.
     */
    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
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
    fn expression(&mut self) -> Result<Expr, LoxError> {
        // Parse and return the equality
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.or()?;

        if self.matchs_next(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(var_expr) = expr {
                let name = var_expr.name;
                return Ok(Expr::Assign(AssignExpr {
                    name,
                    value: Box::new(value),
                }));
            }

            return Err(LoxError::Parser {
                token: equals,
                error_type: ParserErrorType::InvalidAssignTarget,
                msg: "".to_string(),
            });
        }

        Ok(expr)
    }

    /**
     * Parses the next token into an '!=' or '==' expression.
     */
    fn equality(&mut self) -> Result<Expr, LoxError> {
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
    fn comparison(&mut self) -> Result<Expr, LoxError> {
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
    fn term(&mut self) -> Result<Expr, LoxError> {
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
    fn factor(&mut self) -> Result<Expr, LoxError> {
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

    fn or(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.and()?;

        while self.matchs_next(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;

            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.equality()?;

        while self.matchs_next(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;

            expr = Expr::Logical(LogicalExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    /**
     * Parses the next tokens as a unary expression.
     *
     * Note: It can start with '!' or '-', like '-4' or '!true'.
     */
    fn unary(&mut self) -> Result<Expr, LoxError> {
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
    fn primary(&mut self) -> Result<Expr, LoxError> {
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
        Err(LoxError::Parser {
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
    fn consume(&mut self, ttype: TokenType, msg: &str) -> Result<Token, LoxError> {
        // Check that the next token as the correct type
        if self.check(ttype) {
            return Ok(self.advance());
        }

        // Error out with the given message string
        Err(LoxError::Parser {
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
