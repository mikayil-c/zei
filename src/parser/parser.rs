use crate::error::report::*;
use crate::lexer::token::*;
use crate::parser::expr::*;

#[derive(Clone, Debug)]
pub struct Parser<'a> {
    pub tokens: &'a Vec<Token<'a>>,
    pub current: usize,
    pub diagnostics: Vec<Diagnostic>,
}

// Parser has a structure that pushes errors to the top
// and continues from the next most logical part.
impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
            diagnostics: vec![],
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt<'a>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(_) => {
                    self.synchronize();
                }
            }
        }

        statements
    }

    fn declaration(&mut self) -> Result<Stmt<'a>, ParseError> {
        if self.match_token(&[TokenType::Var]) {
            return Ok(self.var_declaration()?);
        }
        Ok(self.statement()?)
    }

    fn var_declaration(&mut self) -> Result<Stmt<'a>, ParseError> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();
        let mut initializer = Expr::Literal {
            value: LiteralType::None,
        };

        if self.match_token(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }
        self.consume(
            TokenType::Semicolon,
            "Except ';' after variable declaration",
        )?;
        Ok(Stmt::Var {
            name: name,
            initializer: initializer,
        })
    }

    fn statement(&mut self) -> Result<Stmt<'a>, ParseError> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt<'a>, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        Ok(Stmt::Print { expression: expr })
    }

    fn expression_statement(&mut self) -> Result<Stmt<'a>, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        Ok(Stmt::Expression { expression: expr })
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

        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous().clone(),
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

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token<'a>, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        let token = self.peek().clone();
        Err(self.error(&token, message))
    }

    // This section is just a function written to call the stack.
    // The `ParseError` struct is empty.
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

    fn synchronize(&mut self) {
        self.advance();

        loop {
            if self.is_at_end() {
                return;
            }

            // If the previous token is ‘;’, that means we are already
            // at the beginning of a new expression.
            if self.previous().token_type == TokenType::Semicolon {
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
