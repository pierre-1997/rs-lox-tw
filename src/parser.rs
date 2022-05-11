use crate::errors::ParserError;
use crate::expr::*;
use crate::token::{Object, Token};
use crate::token_type::TokenType;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(e) => {
                eprintln!("{}", e);
                None
            }
        }
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while self.matchs_next(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();

            let right = self.comparison()?;
            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while self.matchs_next(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while self.matchs_next(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while self.matchs_next(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;

            expr = Expr::Binary(BinaryExpr {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.matchs_next(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;

            return Ok(Expr::Unary(UnaryExpr {
                operator,
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.matchs_next(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::False),
            }));
        }

        if self.matchs_next(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::True),
            }));
        }

        if self.matchs_next(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Some(Object::Nil),
            }));
        }

        if self.matchs_next(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal,
            }));
        }

        if self.matchs_next(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression.")?;
            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }

        Err(ParserError::ExpectedExpression)
    }

    fn consume(&mut self, ttype: TokenType, msg: &str) -> Result<Token, ParserError> {
        if self.check(ttype) {
            return Ok(self.advance());
        }

        eprintln!("{}", msg);
        Err(ParserError::InvalidConsumeType)
    }

    fn matchs_next(&mut self, types: &[TokenType]) -> bool {
        for ttype in types {
            if self.check(*ttype) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().ttype == ttype
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().ttype == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].dup()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].dup()
    }

    fn synchronise(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ttype == TokenType::Semicolon {
                return;
            }

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

            self.advance();
        }
    }
}
