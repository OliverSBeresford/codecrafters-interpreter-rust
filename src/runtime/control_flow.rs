use crate::runtime::runtime_error::RuntimeError;
use crate::runtime::value::Value;

#[derive(Debug)]
pub enum ControlFlow {
    Return(Value),
    RuntimeError(RuntimeError),
}
