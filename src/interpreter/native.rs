use std::{
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    error::report::Diagnostic,
    interpreter::{callable::Callable, interpreter::Interpreter, value::Value},
};

pub struct Clock;

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn clock>")
    }
}

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Value>,
    ) -> Result<Value, Diagnostic> {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("System time is not valid");

        Ok(Value::Number(since_the_epoch.as_secs_f64()))
    }
}
