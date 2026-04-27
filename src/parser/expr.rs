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
    Variable {
        name: Token<'a>,
    },
}

#[derive(Clone, Debug)]
pub enum Stmt<'a> {
    Expression {
        expression: Expr<'a>,
    },
    Print {
        expression: Expr<'a>,
    },
    Var {
        name: Token<'a>,
        initializer: Expr<'a>,
    },
}
