use crate::lexer::models::{LiteralType, Token};

#[derive(Clone, Debug)]
// Expression nodes for the syntax tree.
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Get {
        object: Box<Expr>,
        name: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralType,
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Set {
        object: Box<Expr>,
        name: Token,
        value: Box<Expr>,
    },
    This {
        keyword: Token,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
}

#[derive(Clone, Debug)]
// Statement nodes for the syntax tree.
pub enum Stmt {
    Block {
        statements: Vec<Stmt>,
    },
    Break {
        keyword: Token,
    },
    Class {
        name: Token,
        methods: Vec<Stmt>,
    },
    Continue {
        keyword: Token,
    },
    Expression {
        expression: Expr,
    },
    For {
        initializer: Option<Box<Stmt>>,
        condition: Expr,
        increment: Option<Expr>,
        body: Box<Stmt>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    Print {
        expression: Expr,
    },
    Return {
        keyword: Token,
        value: Expr,
    },
    Var {
        name: Token,
        initializer: Expr,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
}
