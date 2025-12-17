use std::fmt::Debug;

use crate::runtime::control_flow::ControlFlow;
use crate::runtime::interpreter::Interpreter;
use crate::runtime::value::Value;

pub trait Callable: Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, ControlFlow>;
    fn to_string(&self) -> String;
    fn name(&self) -> &str;
}
