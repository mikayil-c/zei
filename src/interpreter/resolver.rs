use std::collections::HashMap;

use crate::{
    error::report::{Diagnostic, DiagnosticKind, Span},
    interpreter::interpreter::Interpreter,
    lexer::models::{LiteralType, Token},
    parser::models::{Expr, Stmt},
};

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
        }
    }

    pub fn resolve(&mut self, statements: &[Stmt]) -> Result<(), Diagnostic> {
        for statement in statements {
            self.resolve_stmt(&statement)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) -> Result<(), Diagnostic> {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve(&statements)?;
                self.end_scope();
                Ok(())
            }
            Stmt::Expression { expression } => todo!(),
            Stmt::Function { name, params, body } => {
                self.declare(&name);
                self.define(&name);

                resolve_function(&stmt);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => todo!(),
            Stmt::Print { expression } => todo!(),
            Stmt::Return { keyword, value } => todo!(),
            Stmt::Var { name, initializer } => {
                self.declare(&name);

                if name.literal != LiteralType::None {
                    self.resolve_expr(&initializer)?;
                }

                self.define(&name);
                Ok(())
            }
            Stmt::While { condition, body } => todo!(),
        }
    }

    fn resolve_function(&mut self, function: &Stmt) -> Result<(), Diagnostic> {
        self.begin_scope();
        match function {
            Stmt::Function { name, params, body } => Ok(()),
            _ => Ok(()),
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), Diagnostic> {
        match expr {
            Expr::Assign { name, value } => {
                self.resolve_expr(&value)?;
                self.resolve_local(&expr, &name)?;
                return Ok(());
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => todo!(),
            Expr::Call {
                callee,
                paren,
                arguments,
            } => todo!(),
            Expr::Grouping { expression } => todo!(),
            Expr::Literal { value } => todo!(),
            Expr::Logical {
                left,
                operator,
                right,
            } => todo!(),
            Expr::Unary { operator, right } => todo!(),
            Expr::Variable { name } => {
                if !self.scopes.is_empty() && self.scopes.last() {
                    
                }
            },
        }
        Ok(())
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) -> Result<(), Diagnostic> {
        let mut i = self.scopes.len() - 1;
        loop {
            if i < 0 {
                return Ok(());
            }

            let a = self.scopes.get(i);

            if a.unwrap().contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, self.scopes.len());
                return Ok(());
            }
            i -= 1;
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) -> Result<(), Diagnostic> {
        if self.scopes.is_empty() {
            return Ok(());
        }

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.to_string(), false);
        };

        Ok(())
    }

    fn define(&mut self, name: &Token) -> Result<(), Diagnostic> {
        if self.scopes.is_empty() {
            return Ok(());
        }

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.to_string(), true);
        };
        Ok(())
    }
}
