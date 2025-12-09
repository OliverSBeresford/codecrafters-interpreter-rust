use crate::callable::Callable;
use crate::runtime_error::RuntimeError;
use crate::value::Value;
use crate::interpreter::Interpreter;
use crate::statement_syntax_tree::{Statement, StatementRef};
use crate::environment::{Environment, EnvRef};

pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<StatementRef>,
}

impl Function {
    // Create a Function from a Statement::Function
    pub fn from_statement(stmt: StatementRef) -> Result<Self, RuntimeError> {
        if let Statement::Function { name, params, body } = &*stmt {
            Ok(Function {
                name: name.lexeme.clone(),
                params: params.iter().map(|param| param.lexeme.clone()).collect(),
                body: body.clone(),
            })
        } else {
            // This should not happen if used correctly (even if the user makes a mistake)
            Err(RuntimeError::new(0, "Expected a function statement.".to_string()))
        }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value {
        let environment: EnvRef = Environment::new(Some(interpreter.globals.clone()));

        // Loop through params and args simultaneously (using zip) and define them in the new environment
        for (param, arg) in self.params.iter().zip(args.into_iter()) {
            environment.borrow_mut().define(param.clone(), arg);
        }

        let _ = interpreter.execute_block(&self.body, environment);

        Value::Nil
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.name)
    }

    fn name(&self) -> &str {
        &self.name
    }
}
