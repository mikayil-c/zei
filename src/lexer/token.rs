use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
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
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
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
        "else" => Some(TokenType::Else),
        "false" => Some(TokenType::False),
        "for" => Some(TokenType::For),
        "fun" => Some(TokenType::Fun),
        "if" => Some(TokenType::If),
        "nil" => Some(TokenType::Nil),
        "or" => Some(TokenType::Or),
        "print" => Some(TokenType::Print),
        "return" => Some(TokenType::Return),
        "super" => Some(TokenType::Super),
        "this" => Some(TokenType::This),
        "true" => Some(TokenType::True),
        "var" => Some(TokenType::Var),
        "while" => Some(TokenType::While),
        _ => None,
    }
}

#[derive(Clone, Debug)]
pub enum LiteralType {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
}

#[derive(Clone, Debug)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: LiteralType,
    pub span: Span,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, literal: LiteralType, span: Span) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            span,
        }
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?} {:?}",
            self.token_type, self.lexeme, self.literal, self.span
        )
    }
}

#[derive(Clone, Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind, // Error, Warning, Note.
    pub message: String,
    pub span: Span, // Range in source code.
}

impl Diagnostic {
    pub fn new(kind: DiagnosticKind, message: String, span: Span) -> Self {
        Self {
            kind,
            message,
            span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Note,
}
