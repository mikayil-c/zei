use crate::error::report::Span;

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
// Token kinds produced by the lexer and consumed by the parser.
pub enum TokenType {
    // Single character tokens.
    Comma,
    Dot,
    Minus,
    LeftBrace,
    LeftParen,
    Plus,
    RightBrace,
    RightParen,
    Semicolon,
    Slash,
    Star,

    // One or Two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    Number,
    String,

    // Keywords.
    And,
    Class,
    Continue,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Break,
    Super,
    This,
    True,
    Var,
    While,

    // Eof
    Eof,
}

pub fn get_keyword_token(text: &str) -> Option<TokenType> {
    match text {
        "and" => Some(TokenType::And),
        "class" => Some(TokenType::Class),
        "continue" => Some(TokenType::Continue),
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "break" => Some(TokenType::Break),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq)]
// Literal values embedded in tokens and promoted into the AST.
pub enum LiteralType {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
}

#[derive(Clone, Debug)]
// A token with an owned lexeme and span metadata.
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: LiteralType,
    pub span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: LiteralType, span: Span) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            span,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?} {:?}",
            self.token_type, self.lexeme, self.literal, self.span
        )
    }
}
