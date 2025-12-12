use crate::runtime_error::RuntimeError;
use crate::value::Value;

pub enum ControlFlow {
    Return(Value),
    RuntimeError(RuntimeError),
}
