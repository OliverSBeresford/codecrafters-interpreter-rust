use crate::token::Token;

// Define a RuntimeError struct to represent runtime errors during interpretation
pub struct RuntimeError<'a> {
    pub token: Token<'a>,
    pub message: String,
}

impl<'a> RuntimeError<'a> {
    pub fn new(token: Token<'a>, message: String) -> Self {
        return RuntimeError {
            token,
            message,
        };
    }
}