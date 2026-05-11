use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    environment::environment::Environment,
    interpreter::{
        callable::Callable, execution_error::ExecutionError, interpreter::Interpreter, value::Value,
    },
    lexer::models::Token,
    parser::models::Stmt,
};

pub struct Function {
    _name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    ) -> Self {
        Self {
            _name: name,
            params,
            body,
            closure,
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self._name.lexeme)
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, ExecutionError> {
        let function_env = Rc::new(RefCell::new(Environment::new_with_enclosing(Rc::clone(
            &self.closure,
        ))));

        for (param, arg) in self.params.iter().zip(args.into_iter()) {
            function_env
                .borrow_mut()
                .define(param.lexeme.to_string(), arg);
        }

        match interpreter.execute_block(&self.body, function_env) {
            Ok(_) => Ok(Value::Nil),
            Err(err) => match err {
                ExecutionError::Return(value) => Ok(value),
                ExecutionError::Error(diagnostic) => Err(ExecutionError::Error(diagnostic)),
            },
        }
    }
}
