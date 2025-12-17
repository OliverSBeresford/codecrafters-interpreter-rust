use std::fmt;

// Define a RuntimeError struct to represent runtime errors during interpretation
pub struct RuntimeError {
    pub line: usize,
    pub message: String,
}

impl RuntimeError {
    pub fn new(line: usize, message: String) -> Self {
        RuntimeError { line, message }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] RuntimeError: {}", self.line, self.message)
    }
}
