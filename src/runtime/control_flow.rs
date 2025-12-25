use crate::runtime::runtime_error::RuntimeError;
use crate::runtime::value::Value;

/// Enum used to represent control flow changes during interpretation, such as returning a value or encountering a runtime error.
#[derive(Debug)]
pub enum ControlFlow {
    Return(Value),
    RuntimeError(RuntimeError),
}
