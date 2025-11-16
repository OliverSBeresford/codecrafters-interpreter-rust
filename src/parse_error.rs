// Define a ParseError struct to represent runtime errors during interpretation
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl ParseError {
    pub fn new(line: usize, message: String) -> Self {
        return ParseError {
            line,
            message,
        };
    }
}
