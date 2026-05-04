use crate::lexer::models::{LiteralType, Token};

#[derive(Clone, Debug)]
// Expression nodes for the syntax tree.
pub enum Expr<'a> {
    Assign {
        name: Token<'a>,
        value: Box<Expr<'a>>,
    },

    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },

    Call {
        callee: Box<Expr<'a>>,
        paren: Token<'a>,
        arguments: Vec<Expr<'a>>,
    },

    Grouping {
        expression: Box<Expr<'a>>,
    },

    Literal {
        value: LiteralType,
    },

    Logical {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
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
// Statement nodes for the syntax tree.
pub enum Stmt<'a> {
    Block {
        statements: Vec<Stmt<'a>>,
    },
    Expression {
        expression: Expr<'a>,
    },
    If {
        condition: Expr<'a>,
        then_branch: Box<Stmt<'a>>,
        else_branch: Option<Box<Stmt<'a>>>,
    },
    Print {
        expression: Expr<'a>,
    },
    Var {
        name: Token<'a>,
        initializer: Expr<'a>,
    },
    While {
        condition: Expr<'a>,
        body: Box<Stmt<'a>>,
    },
}
