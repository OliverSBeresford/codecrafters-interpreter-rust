use std::fmt;

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

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] ParseError: {}", self.line, self.message)
    }
}
