use std::collections::HashMap;

use crate::{
    error::report::{Diagnostic, DiagnosticKind},
    interpreter::value::Value,
    lexer::token::Token,
};

#[derive(Clone, Debug)]
struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    fn get(&self, name: &Token) -> Option<&Value> {
        if self.values.contains_key(name.lexeme) {
            let value = self.values.get(name.lexeme);
            return value;
        }
        None
    }
}
