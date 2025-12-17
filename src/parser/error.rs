use std::fmt;

// ParseError represents syntax errors detected during parsing
pub struct ParseError {
    pub line: usize,
    pub message: String,
}

impl ParseError {
    pub fn new(line: usize, message: String) -> Self {
        ParseError { line, message }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] ParseError: {}", self.line, self.message)
    }
}
