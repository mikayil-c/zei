use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    error::report::{Diagnostic, DiagnosticKind},
    interpreter::value::Value,
    lexer::models::Token,
};

#[derive(Clone, Debug)]
// A scope for variable bindings with an optional enclosing parent.
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        return Self {
            enclosing: None,
            values: HashMap::new(),
        };
    }

    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        return Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        };
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Value, Diagnostic> {
        if let Some(value) = self.values.get(name.lexeme.as_str()) {
            return Ok(value.clone());
        }

        if let Some(environment) = &self.enclosing {
            return environment.borrow().get(name);
        }

        Err(Diagnostic {
            kind: DiagnosticKind::Error,
            message: format!("Undefined variable '{}'.", name.lexeme.as_str()),
            span: name.span.clone(),
        })
    }

    pub fn assign(&mut self, name: &Token, value: Value) -> Result<(), Diagnostic> {
        if let Some(existing_value) = self.values.get_mut(name.lexeme.as_str()) {
            *existing_value = value;
            return Ok(());
        }

        if let Some(environment) = &self.enclosing {
            return environment.borrow_mut().assign(name, value);
        }

        Err(Diagnostic {
            kind: DiagnosticKind::Error,
            message: format!("Undefined variable '{}'.", name.lexeme.as_str()),
            span: name.span.clone(),
        })
    }
}
