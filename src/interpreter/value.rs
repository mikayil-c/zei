use std::fmt;

use crate::lexer::models::LiteralType;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
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
