use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::{
    error::report::{Diagnostic, DiagnosticKind, Span},
    interpreter::{
        callable::Callable,
        execution_error::ExecutionError,
        function::Function,
        interpreter::Interpreter,
        value::Value,
    },
    lexer::models::Token,
};

#[derive(Clone)]
pub struct Class {
    name: Token,
    methods: HashMap<String, Rc<Function>>,
}

impl Class {
    pub fn new(name: Token, methods: HashMap<String, Rc<Function>>) -> Self {
        Self { name, methods }
    }

    pub fn find_method(&self, name: &str) -> Option<Rc<Function>> {
        self.methods.get(name).cloned()
    }

    pub fn class_arity(&self) -> usize {
        self.find_method("init").map_or(0, |method| method.arity())
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<class {}>", self.name.lexeme)
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        self.class_arity()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, ExecutionError> {
        let instance = Rc::new(RefCell::new(Instance::new(Rc::new(self.clone()))));

        if let Some(initializer) = self.find_method("init") {
            let bound = initializer.bind(Value::Instance(Rc::clone(&instance)))?;
            if let Value::Callable(function) = bound {
                function.call(interpreter, args)?;
            }
        }

        Ok(Value::Instance(instance))
    }

    fn bind(&self, _instance: Value) -> Result<Value, ExecutionError> {
        Err(ExecutionError::Error(Diagnostic {
            kind: DiagnosticKind::Error,
            message: "Classes cannot be bound as values.".to_string(),
            span: Span::new(0, 0, 0, 0),
        }))
    }
}

#[derive(Clone)]
pub struct Instance {
    class: Rc<Class>,
    fields: HashMap<String, Value>,
}

impl Instance {
    pub fn new(class: Rc<Class>) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        self.fields.get(name).cloned()
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.fields.insert(name, value);
    }

    pub fn class(&self) -> Rc<Class> {
        Rc::clone(&self.class)
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<instance {}>", self.class.name.lexeme)
    }
}
