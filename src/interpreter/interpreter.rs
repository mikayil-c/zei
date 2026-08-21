use std::cell::RefCell;
use std::rc::Rc;

use crate::environment::environment::Environment;
use crate::error::report::*;
use crate::interpreter::callable::Callable;
use crate::interpreter::class::Class;
use crate::interpreter::execution_error::ExecutionError;
use crate::interpreter::function::Function;
use crate::interpreter::native::Clock;
use crate::interpreter::value::Value;
use crate::lexer::models::{LiteralType, Token, TokenType};
use crate::parser::models::{Expr, Stmt};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));

        globals
            .borrow_mut()
            .define("clock".to_string(), Value::Callable(Rc::new(Clock)));

        Interpreter {
            environment: Rc::clone(&globals),
        }
    }

    // Execute each statement independently so one runtime error does not stop the rest.
    pub fn interpret(&mut self, statements: &Vec<Stmt>, source: &str) -> bool {
        let mut had_error = false;

        for stmt in statements {
            match self.execute(&stmt) {
                Ok(_) => (),
                Err(execution_error) => match execution_error {
                    ExecutionError::Error(diagnostic) => {
                        had_error = true;
                        report_diagnostics(source, &[diagnostic]);
                    }
                    ExecutionError::Return(_) => {}
                },
            }
        }

        !had_error
    }

    // Statements produce side effects; expressions are evaluated for their values.
    fn execute(&mut self, stmt: &Stmt) -> Result<(), ExecutionError> {
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

                self.environment
                    .borrow_mut()
                    .define(name.lexeme.to_string(), value);

                Ok(())
            }
            Stmt::Block { statements } => {
                let block_env = Rc::new(RefCell::new(Environment::new_with_enclosing(Rc::clone(
                    &self.environment,
                ))));
                self.execute_block(statements, block_env)
            }
            Stmt::Class { name, methods } => {
                let mut class_methods = std::collections::HashMap::new();

                for method in methods {
                    if let Stmt::Function { name, params, body } = method {
                        let function = Rc::new(Function::new(
                            name.clone(),
                            params.clone(),
                            body.clone(),
                            Rc::clone(&self.environment),
                        ));
                        class_methods.insert(name.lexeme.to_string(), function);
                    }
                }

                let class = Rc::new(Class::new(name.clone(), class_methods));
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.to_string(), Value::Callable(class.clone()));
                Ok(())
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let v = self.evaluate(condition)?;

                if v.is_truthy() {
                    self.execute(then_branch)?
                } else if let Some(else_stmt) = else_branch {
                    self.execute(else_stmt)?
                }

                Ok(())
            }
            Stmt::While { condition, body } => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?
                }

                Ok(())
            }
            Stmt::Function { name, params, body } => {
                let function = Function::new(
                    name.clone(),
                    params.clone(),
                    body.clone(),
                    Rc::clone(&self.environment),
                );

                self.environment
                    .borrow_mut()
                    .define(name.lexeme.to_string(), Value::Callable(Rc::new(function)));

                Ok(())
            }
            Stmt::Return { keyword: _, value } => {
                let return_value = if let Expr::Literal {
                    value: LiteralType::None,
                } = value
                {
                    Value::Nil
                } else {
                    self.evaluate(value)?
                };

                Err(ExecutionError::Return(return_value))
            }
        }
    }

    pub(crate) fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), ExecutionError> {
        let previous = std::mem::replace(&mut self.environment, environment);

        let result = statements.iter().try_for_each(|stmt| self.execute(stmt));

        self.environment = previous;
        result
    }

    // Evaluate an expression node into a runtime value.
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, ExecutionError> {
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
                            return Err(ExecutionError::Error(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: "Operand must be a number".to_string(),
                                span: operator.span.clone(),
                            }));
                        }
                    }
                    TokenType::Bang => {
                        return Ok(Value::Boolean(!right_value.is_truthy()));
                    }
                    _ => {
                        return Err(ExecutionError::Error(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: "Unkown unary operator".to_string(),
                            span: operator.span.clone(),
                        }));
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

                        _ => Err(ExecutionError::Error(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: "Runtime Error: Operands must be numbers at line {}"
                                .to_string(),
                            span: operator.span.clone(),
                        })),
                    },

                    TokenType::Slash => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),

                        _ => Err(ExecutionError::Error(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: "Runtime Error: Operands must be numbers at line {}"
                                .to_string(),
                            span: operator.span.clone(),
                        })),
                    },

                    TokenType::Star => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),

                        _ => Err(ExecutionError::Error(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: "Runtime Error: Operands must be numbers at line {}"
                                .to_string(),
                            span: operator.span.clone(),
                        })),
                    },

                    TokenType::Plus => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                        (Value::String(l), Value::String(r)) => {
                            Ok(Value::String(format!("{}{}", l, r)))
                        }

                        _ => Err(ExecutionError::Error(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: "Operands must be two numbers or two strings.".to_string(),
                            span: operator.span.clone(),
                        })),
                    },

                    TokenType::Greater => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l > r)),

                        _ => Err(ExecutionError::Error(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: format!(
                                "Runtime Error: Operands must be numbers at line {}",
                                operator.span.line
                            ),
                            span: operator.span.clone(),
                        })),
                    },

                    TokenType::GreaterEqual => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l >= r)),

                        _ => Err(ExecutionError::Error(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: format!(
                                "Runtime Error: Operands must be numbers at line {}",
                                operator.span.line
                            ),
                            span: operator.span.clone(),
                        })),
                    },

                    TokenType::Less => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l < r)),

                        _ => Err(ExecutionError::Error(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: format!(
                                "Runtime Error: Operands must be numbers at line {}",
                                operator.span.line
                            ),
                            span: operator.span.clone(),
                        })),
                    },

                    TokenType::LessEqual => match (left_value, right_value) {
                        (Value::Number(l), Value::Number(r)) => Ok(Value::Boolean(l <= r)),

                        _ => Err(ExecutionError::Error(Diagnostic {
                            kind: DiagnosticKind::Error,
                            message: format!(
                                "Runtime Error: Operands must be numbers at line {}",
                                operator.span.line
                            ),
                            span: operator.span.clone(),
                        })),
                    },

                    TokenType::BangEqual => Ok(Value::Boolean(left_value != right_value)),
                    TokenType::EqualEqual => Ok(Value::Boolean(left_value == right_value)),

                    _ => Err(ExecutionError::Error(Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: "Unknown binary operator".to_string(),
                        span: operator.span.clone(),
                    })),
                }
            }
            Expr::Variable { name } => self
                .environment
                .borrow()
                .get(name)
                .map_err(|diag| ExecutionError::Error(diag)),
            Expr::This { .. } => {
                let this_token = Token::new(
                    TokenType::Identifier,
                    "this".to_string(),
                    LiteralType::None,
                    crate::error::report::Span::new(0, 0, 0, 0),
                );
                self.environment
                    .borrow()
                    .get(&this_token)
                    .map_err(|diag| ExecutionError::Error(diag))
            }
            Expr::Get { object, name } => {
                let object_value = self.evaluate(object)?;
                match object_value {
                    Value::Instance(instance) => {
                        let instance_borrowed = instance.borrow();
                        if let Some(value) = instance_borrowed.get(&name.lexeme) {
                            Ok(value)
                        } else if let Some(method) =
                            instance_borrowed.class().find_method(&name.lexeme)
                        {
                            let bound = method.bind(Value::Instance(instance.clone()))?;
                            Ok(bound)
                        } else {
                            Err(ExecutionError::Error(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: format!("Undefined property '{}'.", name.lexeme),
                                span: name.span.clone(),
                            }))
                        }
                    }
                    _ => Err(ExecutionError::Error(Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: "Only instances have properties.".to_string(),
                        span: name.span.clone(),
                    })),
                }
            }
            Expr::Set {
                object,
                name,
                value,
            } => {
                let object_value = self.evaluate(object)?;
                let evaluated = self.evaluate(value)?;

                if let Value::Instance(instance) = object_value {
                    instance
                        .borrow_mut()
                        .set(name.lexeme.to_string(), evaluated.clone());
                    Ok(evaluated)
                } else {
                    Err(ExecutionError::Error(Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: "Only instances have fields.".to_string(),
                        span: name.span.clone(),
                    }))
                }
            }
            Expr::Assign { name, value } => {
                let evaluated = self.evaluate(value)?;

                self.environment
                    .borrow_mut()
                    .assign(name, evaluated.clone())?;

                Ok(evaluated)
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left_value = self.evaluate(left)?;

                match operator.token_type {
                    TokenType::Or => {
                        if left_value.is_truthy() {
                            Ok(left_value)
                        } else {
                            self.evaluate(right)
                        }
                    }
                    TokenType::And => {
                        if !left_value.is_truthy() {
                            Ok(left_value)
                        } else {
                            self.evaluate(right)
                        }
                    }
                    _ => Err(ExecutionError::Error(Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: "Unknown logical operator".to_string(),
                        span: operator.span,
                    })),
                }
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                let callee_value = self.evaluate(callee)?;

                let mut args: Vec<Value> = vec![];

                for argument in arguments {
                    args.push(self.evaluate(argument)?);
                }

                match callee_value {
                    Value::Callable(func) => {
                        if args.len() != func.arity() {
                            return Err(ExecutionError::Error(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: format!(
                                    "Expected {} arguments but got {}.",
                                    func.arity(),
                                    args.len()
                                ),
                                span: paren.span,
                            }));
                        }
                        func.call(self, args)
                    }
                    _ => Err(ExecutionError::Error(Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: "Can only call functions and classes.".to_string(),
                        span: paren.span,
                    })),
                }
            }
        }
    }
}
