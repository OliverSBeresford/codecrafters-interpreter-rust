use std::fmt::Debug;

use crate::runtime::control_flow::ControlFlow;
use crate::runtime::interpreter::Interpreter;
use crate::runtime::value::Value;

pub trait Callable<'a>: Debug {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter<'a>, args: Vec<Value<'a>>) -> Result<Value<'a>, ControlFlow<'a>>;
    fn to_string(&self) -> String;
    fn name(&self) -> &str;
}
