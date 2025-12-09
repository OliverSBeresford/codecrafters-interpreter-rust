use crate::callable::Callable;
use crate::value::Value;
use crate::interpreter::Interpreter;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut Interpreter, _args: Vec<Value>) -> Value {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Value::Float(now.as_secs_f64())
    }

    fn to_string(&self) -> String {
        "<native fn clock>".to_string()
    }

    fn name(&self) -> &str {
        "clock"
    }
}