use crate::callable::Callable;
use std::rc::Rc;

// Define a Value enum to represent evaluated values, can be anything because Lox is dynamically typed
#[derive(Clone)]
pub enum Value {
    Callable(Rc<dyn Callable>),
    Integer(isize),
    Float(f64),
    Str(String),
    Bool(bool),
    Nil,
}