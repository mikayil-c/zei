use crate::error::report::*;
use crate::interpreter::value::{self, Value};
use crate::lexer::token::TokenType;
use crate::parser::expr::Expr;

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&mut self, expression: &Expr, source: &str) {
        match self.evaluate(expression) {
            Ok(value) => {
                println!("{}", value);
            }
            Err(diagnostic) => {
                report_diagnostics(source, &[diagnostic]);
            }
        };
    }

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
        }
    }
}
