use crate::{error::report::Diagnostic, interpreter::value::Value};

pub enum ExecutionError {
    Break,
    Continue,
    Error(Diagnostic),
    Return(Value),
}

impl From<Diagnostic> for ExecutionError {
    fn from(diag: Diagnostic) -> Self {
        ExecutionError::Error(diag)
    }
}
