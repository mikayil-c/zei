use crate::interpreter::{execution_error::ExecutionError, interpreter::Interpreter, value::Value};
use std::fmt;

pub trait Callable: fmt::Display {
    // Number of arguments expected by this callable.
    fn arity(&self) -> usize;

    // Execute the callable and return a runtime value or diagnostic.
    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, ExecutionError>;

    fn bind(&self, _instance: Value) -> Result<Value, ExecutionError> {
        Err(ExecutionError::Error(crate::error::report::Diagnostic {
            kind: crate::error::report::DiagnosticKind::Error,
            message: "This callable cannot be bound to an instance.".to_string(),
            span: crate::error::report::Span::new(0, 0, 0, 0),
        }))
    }
}
