use crate::environment::environment::Environment;
use crate::error::report::*;
use crate::interpreter::value::Value;
use crate::lexer::models::TokenType;
use crate::parser::models::{Expr, Stmt};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    // Execute each statement independently so one runtime error does not stop the rest.
    pub fn interpret(&mut self, statements: &Vec<Stmt>, source: &str) {
        for stmt in statements {
            match self.execute(&stmt) {
                Ok(_) => (),
                Err(diagnostic) => {
                    report_diagnostics(source, &[diagnostic]);
                }
            }
        }
    }

    // Statements produce side effects; expressions are evaluated for their values.
    fn execute(&mut self, stmt: &Stmt) -> Result<(), Diagnostic> {
        match stmt {
            Stmt::Expression { expression } => {
                self.evaluate(&expression)?;
                Ok(())
            }
            Stmt::Print { expression } => {
                let value = self.evaluate(expression)?;
                println!("{}", value);
                Ok(())
            }
            Stmt::Var { name, initializer } => {
                let value = self.evaluate(initializer)?;

                self.environment.define(name.lexeme.to_string(), value);
                Ok(())
            }
            Stmt::Block { statements } => {
                let block_env = Environment::new_with_enclosing(self.environment.clone());
                self.execute_block(statements, block_env)
            }
        }
    }

    fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), Diagnostic> {
        let previous = std::mem::replace(&mut self.environment, environment);

        let result = statements.iter().try_for_each(|stmt| self.execute(stmt));

        self.environment = previous;
        result
    }

    // Evaluate an expression node into a runtime value.
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, Diagnostic> {
        match expr {
            Expr::Literal { value } => Ok(value.clone().into()),
            Expr::Grouping { expression } => self.evaluate(expression),
            Expr::Unary { operator, right } => {
                let right_value = self.evaluate(right)?;

                match operator.token_type {
                    TokenType::Minus => {
                        if let Value::Number(n) = right_value {
                            return Ok(Value::Number(-n));
                        } else {
                            return Err(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: "Operand must be a number".to_string(),
                                span: operator.span.clone(),
                            });
                        }
                    }
                    TokenType::Bang => {
                        return Ok(Value::Boolean(right_value.is_truthy()));
                    }
                    _ => {
                        return Err(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: "Unkown unary operator".to_string(),
                            span: operator.span.clone(),
                        });
                    }
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate(left)?;
                let right_value = self.evaluate(right)?;

                match operator.token_type {
                    TokenType::Minus => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),

                        _ => Err(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: "Runtime Error: Operands must be numbers at line {}"
                                .to_string(),
                            span: operator.span.clone(),
                        }),
                    },

                    TokenType::Slash => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),

                        _ => Err(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: "Runtime Error: Operands must be numbers at line {}"
                                .to_string(),
                            span: operator.span.clone(),
                        }),
                    },

                    TokenType::Star => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),

                        _ => Err(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: "Runtime Error: Operands must be numbers at line {}"
                                .to_string(),
                            span: operator.span.clone(),
                        }),
                    },

                    TokenType::Plus => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                        (Value::String(l), Value::String(r)) => {
                            Ok(Value::String(format!("{}{}", l, r)))
                        }

                        _ => Err(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: "Operands must be two numbers or two strings.".to_string(),
                            span: operator.span.clone(),
                        }),
                    },

                    TokenType::Greater => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),

                        _ => Err(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: format!(
                                "Runtime Error: Operands must be numbers at line {}",
                                operator.span.line
                            ),
                            span: operator.span.clone(),
                        }),
                    },

                    TokenType::GreaterEqual => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),

                        _ => Err(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: format!(
                                "Runtime Error: Operands must be numbers at line {}",
                                operator.span.line
                            ),
                            span: operator.span.clone(),
                        }),
                    },

                    TokenType::Less => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),

                        _ => Err(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: format!(
                                "Runtime Error: Operands must be numbers at line {}",
                                operator.span.line
                            ),
                            span: operator.span.clone(),
                        }),
                    },

                    TokenType::LessEqual => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),

                        _ => Err(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: format!(
                                "Runtime Error: Operands must be numbers at line {}",
                                operator.span.line
                            ),
                            span: operator.span.clone(),
                        }),
                    },

                    TokenType::BangEqual => Ok(Value::Boolean(left_value != right_value)),
                    TokenType::EqualEqual => Ok(Value::Boolean(left_value == right_value)),

                    _ => Err(Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: "Unknown binary operator".to_string(),
                        span: operator.span.clone(),
                    }),
                }
            }
            Expr::Variable { name } => self.environment.get(name),
            Expr::Assign { name, value: _ } => {
                let value = self.evaluate(expr)?;
                self.environment.assign(name, value.clone())?;
                Ok(value)
            }
        }
    }
}
