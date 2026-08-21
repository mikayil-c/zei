use std::collections::HashMap;

use crate::{
    error::report::{Diagnostic, DiagnosticKind},
    interpreter::interpreter::Interpreter,
    lexer::models::Token,
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
                self.resolve(statements)?;
                self.end_scope();
                Ok(())
            }
            Stmt::Expression { expression } => self.resolve_expr(expression),
            Stmt::Function { name, params, body } => {
                let _ = self.declare(&name);
                let _ = self.define(&name);

                self.resolve_function(params, body)
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(then_branch)?;
                if let Some(else_branch) = else_branch {
                    self.resolve_stmt(else_branch)?;
                }
                Ok(())
            }
            Stmt::Print { expression } => self.resolve_expr(expression),
            Stmt::Return { keyword: _, value } => self.resolve_expr(value),
            Stmt::Var { name, initializer } => {
                let _ = self.declare(&name);

                if let Some(scope) = self.scopes.last() {
                    if matches!(scope.get(&name.lexeme), Some(false)) {
                        return Err(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: "Can't read local variable in its own initializer."
                                .to_string(),
                            span: name.span.clone(),
                        });
                    }
                }

                self.resolve_expr(initializer)?;
                let _ = self.define(&name);
                Ok(())
            }
            Stmt::While { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve_stmt(body)
            }
            Stmt::Class { name, methods } => {
                let _ = self.declare(name);
                let _ = self.define(name);

                self.begin_scope();
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert("this".to_string(), true);
                }

                for method in methods {
                    if let Stmt::Function { params, body, .. } = method {
                        self.resolve_function(params, body)?;
                    }
                }

                self.end_scope();
                Ok(())
            }
        }
    }

    fn resolve_function(&mut self, params: &[Token], body: &[Stmt]) -> Result<(), Diagnostic> {
        self.begin_scope();
        if let Some(scope) = self.scopes.last_mut() {
            for param in params {
                scope.insert(param.lexeme.to_string(), true);
            }
        }

        self.resolve(body)?;
        self.end_scope();
        Ok(())
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), Diagnostic> {
        match expr {
            Expr::Assign { name, value } => {
                self.resolve_expr(&value)?;
                self.resolve_local(&expr, &name)?;
                Ok(())
            }
            Expr::Binary {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)?;
                Ok(())
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
                self.resolve_expr(callee)?;
                for argument in arguments {
                    self.resolve_expr(argument)?;
                }
                Ok(())
            }
            Expr::Get { object, name: _ } => self.resolve_expr(object),
            Expr::Grouping { expression } => self.resolve_expr(expression),
            Expr::Literal { value: _ } => Ok(()),
            Expr::Logical {
                left,
                operator: _,
                right,
            } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)
            }
            Expr::This { keyword } => self.resolve_local(expr, keyword),
            Expr::Set {
                object,
                name: _,
                value,
            } => {
                self.resolve_expr(value)?;
                self.resolve_expr(object)
            }
            Expr::Unary { operator: _, right } => self.resolve_expr(right),
            Expr::Variable { name } => {
                if let Some(scope) = self.scopes.last() {
                    if matches!(scope.get(&name.lexeme), Some(false)) {
                        return Err(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: "Can't read local variable in its own initializer."
                                .to_string(),
                            span: name.span.clone(),
                        });
                    }
                }

                self.resolve_local(expr, name)
            }
        }
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) -> Result<(), Diagnostic> {
        let _ = expr;

        for scope in self.scopes.iter().rev() {
            if scope.contains_key(&name.lexeme) {
                return Ok(());
            }
        }

        Ok(())
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
