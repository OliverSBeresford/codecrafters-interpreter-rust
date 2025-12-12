use crate::control_flow::ControlFlow;
use crate::interpreter::Interpreter;
use crate::value::Value;
use std::fmt::Debug;

pub trait Callable: Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, ControlFlow>;
    fn to_string(&self) -> String;
    fn name(&self) -> &str;
}
