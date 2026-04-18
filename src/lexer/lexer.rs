use crate::lexer::report::*;
use crate::lexer::token::*;

pub struct Lexer<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: usize,
    pub diagnostics: Vec<Diagnostic>,
    line_start: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            diagnostics: vec![],
            line_start: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            // This is the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        // End Of File
        self.tokens.push(Token::new(
            TokenType::Eof,
            "",
            LiteralType::None,
            Span {
                start: self.current,
                end: self.current,
                line: self.line,
                column: 0,
            },
        ));

        std::mem::take(&mut self.tokens)
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, LiteralType::None),
            ')' => self.add_token(TokenType::RightParen, LiteralType::None),
            '{' => self.add_token(TokenType::LeftBrace, LiteralType::None),
            '}' => self.add_token(TokenType::RightBrace, LiteralType::None),
            ',' => self.add_token(TokenType::Comma, LiteralType::None),
            '.' => self.add_token(TokenType::Dot, LiteralType::None),
            '-' => self.add_token(TokenType::Minus, LiteralType::None),
            '+' => self.add_token(TokenType::Plus, LiteralType::None),
            ';' => self.add_token(TokenType::Semicolon, LiteralType::None),
            '*' => self.add_token(TokenType::Star, LiteralType::None),
            '!' => {
                let t = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(t, LiteralType::None);
            }
            '=' => {
                let t = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(t, LiteralType::None);
            }
            '<' => {
                let t = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(t, LiteralType::None);
            }
            '>' => {
                let t = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(t, LiteralType::None);
            }

            // Comment line control
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, LiteralType::None);
                }
            }

            ' ' | '\r' | '\t' => {} // Skip spaces
            '\n' => {
                self.line += 1;
                self.line_start = self.current;
            } // Increment line

            '"' => self.string(),

            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.diagnostics.push(Diagnostic::new(
                        DiagnosticKind::Error,
                        format!("Unexpected character: '{}'", c),
                        Span {
                            start: self.start,
                            end: self.current,
                            line: self.line,
                            column: self.start - self.line_start,
                        },
                    ));
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        if c != '\0' {
            // Increment current by exact char size that allocated in memory.
            self.current += c.len_utf8();
        }
        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: LiteralType) {
        let lexeme = &self.source[self.start..self.current];

        let span = Span {
            start: self.start,
            end: self.current,
            line: self.line,
            column: self.start - self.line_start,
        };

        self.tokens
            .push(Token::new(token_type, lexeme, literal, span));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() as char != expected {
            return false;
        }
        self.current += expected.len_utf8();
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current..].chars().next().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        let mut chars = self.source[self.current..].chars();
        chars.next(); // Skip the first char (current).
        chars.next().unwrap_or('\0') // Take second.
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.line_start += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.diagnostics.push(Diagnostic::new(
                DiagnosticKind::Error,
                format!("[line {}] Error: Unterminated string.", self.line),
                Span {
                    start: self.start,
                    end: self.current,
                    line: self.line,
                    column: self.start - self.line_start,
                },
            ));

            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, LiteralType::String(value.to_string()));
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic()
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: f64 = match self.source[self.start..self.current].parse() {
            Ok(v) => v,
            Err(_) => {
                self.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::Error,
                    format!("[line {}] Error: Invalid number format.", self.line),
                    Span {
                        start: self.start,
                        end: self.current,
                        line: self.line,
                        column: self.start - self.line_start,
                    },
                ));
                return;
            }
        };

        self.add_token(TokenType::Number, LiteralType::Number(value));
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = get_keyword_token(text).unwrap_or(TokenType::Identifier);

        self.add_token(token_type, LiteralType::None);
    }
}
