use crate::value::Value;
use std::collections::HashMap;
use crate::runtime_error::RuntimeError;

pub struct Environment {
    // Stores enclosing environment (if any)
    enclosing: Option<Box<Environment>>,

    // Stores variable names and their associated values
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new(enclosing: Option<Box<Environment>>) -> Self {
        return Environment {
            enclosing,
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str, line: usize) -> Result<&Value, RuntimeError> {
        let result = self.values.get(name);

        // If the variable is not found, return an error
        if let None = result {
            return Err(RuntimeError::new(line, format!("Undefined variable '{}'.", name)));
        }
        return Ok(result.unwrap());
    }

    pub fn assign(&mut self, name: &str, value: Value, line: usize) -> Result<(), RuntimeError> {
        if !self.values.contains_key(name) {
            // You can only assign variables that are already defined
            return Err(RuntimeError::new(line, format!("Undefined variable '{}'.", name)));
        }
        self.values.insert(name.to_string(), value);

        // Return success (this function only has side effects)
        Ok(())
    }
}