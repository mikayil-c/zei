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
}
