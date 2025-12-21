use std::time::{SystemTime, UNIX_EPOCH};

use crate::runtime::callable::Callable;
use crate::runtime::control_flow::ControlFlow;
use crate::runtime::interpreter::Interpreter;
use crate::runtime::value::Value;

#[derive(Debug)]
pub struct Clock;

impl<'a> Callable<'a> for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _interpreter: &mut Interpreter<'a>, _args: Vec<Value<'a>>) -> Result<Value<'a>, ControlFlow<'a>> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Ok(Value::Float(now.as_secs_f64()))
    }

    fn to_string(&self) -> String {
        "<native fn clock>".to_string()
    }

    fn name(&self) -> &str {
        "clock"
    }
}
