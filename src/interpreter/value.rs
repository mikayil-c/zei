use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    interpreter::{callable::Callable, class::Instance},
    lexer::models::LiteralType,
};

#[derive(Clone)]
// Runtime values produced by expression evaluation.
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Callable(Rc<dyn Callable>),
    Instance(Rc<RefCell<Instance>>),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Boolean(value) => *value,
            _ => true,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),

            Value::Number(n) => write!(f, "{}", n),

            Value::Boolean(b) => write!(f, "{}", b),

            Value::String(s) => write!(f, "{}", s),
            Value::Callable(c) => write!(f, "{}", c.as_ref()),
            Value::Instance(instance) => write!(f, "{}", instance.borrow()),
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Number(n) => write!(f, "Number({})", n),
            Value::Boolean(b) => write!(f, "Boolean({})", b),
            Value::String(s) => write!(f, "String({})", s),
            Value::Callable(_) => write!(f, "Callable"),
            Value::Instance(instance) => write!(f, "Instance({})", instance.borrow()),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Callable(_), Value::Callable(_)) => false, // karsilastirilamaz
            (Value::Instance(a), Value::Instance(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl From<LiteralType> for Value {
    fn from(literal: LiteralType) -> Self {
        match literal {
            LiteralType::Number(value) => Value::Number(value),
            LiteralType::String(value) => Value::String(value),
            LiteralType::Boolean(value) => Value::Boolean(value),
            LiteralType::None => Value::Nil,
        }
    }
}
