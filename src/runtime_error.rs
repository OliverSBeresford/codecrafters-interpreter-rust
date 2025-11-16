// Define a RuntimeError struct to represent runtime errors during interpretation
pub struct RuntimeError {
    pub line: usize,
    pub message: String,
}

impl RuntimeError {
    pub fn new(line: usize, message: String) -> Self {
        return RuntimeError {
            line,
            message,
        };
    }
}