use std::rc::Rc;

use crate::runtime::callable::Callable;

// Define a Value enum to represent evaluated values, can be anything because Lox is dynamically typed
#[derive(Debug, Clone)]
pub enum Value {
    Callable(Rc<dyn Callable>),
    Integer(isize),
    Float(f64),
    Str(String),
    Bool(bool),
    Nil,
}
