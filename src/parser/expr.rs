use crate::lexer::token::{LiteralType, Token};

#[derive(Clone, Debug)]
pub enum Expr<'a> {
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },

    Grouping {
        expression: Box<Expr<'a>>,
    },

    Literal {
        value: LiteralType,
    },

    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
}
