use crate::runtime::runtime_error::RuntimeError;
use crate::runtime::value::Value;

pub enum ControlFlow<'a> {
    Return(Value<'a>),
    RuntimeError(RuntimeError),
}

impl From<RuntimeError> for ControlFlow<'_> {
    fn from(err: RuntimeError) -> Self {
        ControlFlow::RuntimeError(err)
    }
}
