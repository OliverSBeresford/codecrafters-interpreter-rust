use crate::interpreter::Interpreter;
use crate::value::Value;

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Value;
    fn to_string(&self) -> String;
    fn name(&self) -> &str;
}
