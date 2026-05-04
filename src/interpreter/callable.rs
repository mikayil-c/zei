use crate::{
    error::report::Diagnostic,
    interpreter::{interpreter::Interpreter, value::Value},
};

pub trait Callable {
    // Number of arguments expected by this callable.
    fn arity(&self) -> usize;

    // Execute the callable and return a runtime value or diagnostic.
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, Diagnostic>;
}
