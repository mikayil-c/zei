use crate::lexer::report::*;
use crate::lexer::token::*;
use crate::parser::expr::*;

#[derive(Clone, Debug)]
pub struct Parser<'a> {
    pub tokens: &'a Vec<Token<'a>>,
    pub current: usize,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
            diagnostics: vec![],
        }
    }

    pub fn parse(&mut self) -> Option<Expr<'a>> {
        match self.expression() {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    fn expression(&mut self) -> Result<Expr<'a>, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr<'a>, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr<'a>, ParseError> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr<'a>, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr<'a>, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expr = Expr::Binary {
                left: Box::new(expr),
                operator: operator,
                right: Box::new(right),
            }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr<'a>, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator: operator,
                right: Box::new(right),
            });
        }
        Ok(self.primary()?)
    }

    fn primary(&mut self) -> Result<Expr<'a>, ParseError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal {
                value: LiteralType::Boolean(false),
            });
        }

        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal {
                value: LiteralType::Boolean(true),
            });
        }

        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal {
                value: LiteralType::None,
            });
        }

        if self.match_token(&[TokenType::Number, TokenType::String]) {
            let prev = self.previous().clone();

            return Ok(Expr::Literal {
                value: prev.literal,
            });
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression();

            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;

            return Ok(Expr::Grouping {
                expression: Box::new(expr?),
            });
        }

        let token = self.peek().clone();
        Err(self.error(&token, "Expect expression."))
    }

    fn error(&mut self, token: &Token<'a>, message: &str) -> ParseError {
        let formatted_message = match token.token_type {
            TokenType::Eof => format!("Error at end: {}", message),
            _ => format!("Error at '{}': {}", token.lexeme, message),
        };

        self.diagnostics.push(Diagnostic::new(
            DiagnosticKind::Error,
            formatted_message,
            token.span,
        ));

        ParseError
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token<'a>, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        let token = self.peek().clone();
        Err(self.error(&token, message))
    }

    fn synchronize(&mut self) {
        self.advance();

        loop {
            if self.is_at_end() {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => (),
            }

            self.advance();
        }
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for &t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.previous()
    }

    fn peek(&self) -> &Token<'a> {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token<'a> {
        &self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}
