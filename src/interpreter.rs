use crate::expr_syntax_tree::{Expr};
use crate::token::{Literal, Token, TokenType};
use crate::runtime_error::RuntimeError;
use std::fmt;

// Define a Value enum to represent evaluated values, can be anything because Lox is dynamically typed
#[derive(Debug, Clone)]
pub enum Value {
    Integer(isize),
    Float(f64),
    Str(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Value::Integer(i) => format!("{}", i),
            Value::Float(n) => {
                // If the value is an integer (no fractional part) print one decimal place
                // Otherwise print the float normally.
                format!("{}", n)
            }
            Value::Str(s) => s.clone(),
            Value::Bool(b) => format!("{}", b),
            Value::Nil => "nil".to_string(),
        };
        write!(f, "{}", out)
    }
}

// Define the output type for interpreter methods
type Output = Result<Value, RuntimeError>;

pub struct Interpreter;

impl Interpreter {
    fn is_truthy(v: &Value) -> bool {
        match v {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }

    // Report an evaluation error
    fn error<T>(token: &Token, message: &str) -> Result<T, RuntimeError> {
        if token.token_type == TokenType::Eof {
            return Err(RuntimeError::new(token.line, format!("Error at end: {}", message)));
        } else {
            return Err(RuntimeError::new(token.line, format!("Error at '{}': {}", token.lexeme, message)));
        }
    }

    fn as_number(operator: &Token, v: &Value) -> Result<f64, RuntimeError> {
        match v {
            Value::Float(n) => Ok(*n),
            Value::Integer(i) => Ok(*i as f64),
            _ => Self::error(operator, &format!("Operand must be a number for {}", operator.lexeme)),
        }
    }

    fn evaluate(&mut self, expression: &Expr) -> Result<Value, RuntimeError> {
        match expression {
            Expr::Binary { left, operator, right } => self.visit_binary(left, operator, right),
            Expr::Literal { value } => self.visit_literal(value),
            Expr::Grouping { expression } => self.visit_grouping(expression),
            Expr::Unary { operator, right } => self.visit_unary(operator, right),
        }
    }

    pub fn interpret(&mut self, expression: &Expr) {
        let result = self.evaluate(expression);

        // Handle the result or runtime error
        match result {
            Ok(value) => println!("{}", value),
            Err(runtime_error) => {
                eprintln!("[Line {}] {}", runtime_error.line, runtime_error.message);
                std::process::exit(70);
            }
        }
    }

    fn visit_binary(&mut self, left: &Expr, operator: &Token, right: &Expr) -> Output {
        let left_value = self.evaluate(left)?;
        let right_value = self.evaluate(right)?;
        let non_numeric = !matches!(left_value, Value::Float(_) | Value::Integer(_)) ||
                              !matches!(right_value, Value::Float(_) | Value::Integer(_));
        let either_floating = matches!(left_value, Value::Float(_)) || matches!(right_value, Value::Float(_));
    
        match operator.token_type {
            TokenType::Plus => {
                // Handle string concatenation
                if non_numeric {
                    let (Value::Str(str_left), Value::Str(str_right)) = (left_value, right_value) else {
                        return Self::error(operator, "Operands must be two numbers or two strings for '+'");
                    };
                    return Ok(Value::Str(format!("{}{}", str_left, str_right)));
                }
                // Handle numeric addition
                else if either_floating {
                    return Ok(Value::Float(Self::as_number(operator, &left_value)? + Self::as_number(operator, &right_value)?));
                } else {
                    let (Value::Integer(num_left), Value::Integer(num_right)) = (left_value, right_value) else {
                        return Self::error(operator, "Operands must be two numbers or two strings for '+'");
                    };
                    return Ok(Value::Integer(num_left + num_right));
                }
            }
            TokenType::Minus => {
                if non_numeric {
                    return Self::error(operator, "Operands must be two numbers for '-'");
                } else if either_floating {
                    return Ok(Value::Float(Self::as_number(operator, &left_value)? - Self::as_number(operator, &right_value)?));
                } else {
                    let (Value::Integer(num_left), Value::Integer(num_right)) = (left_value, right_value) else {
                        return Self::error(operator, "Operands must be two integers for '-'");
                    };
                    return Ok(Value::Integer(num_left - num_right));
                }
            }
            TokenType::Star => {
                if non_numeric {
                    return Self::error(operator, "Operands must be two numbers for '*'");
                } else if either_floating {
                    return Ok(Value::Float(Self::as_number(operator, &left_value)? * Self::as_number(operator, &right_value)?));
                } else {
                    let (Value::Integer(num_left), Value::Integer(num_right)) = (left_value, right_value) else {
                        return Self::error(operator, "Operands must be two integers for '*'");
                    };
                    return Ok(Value::Integer(num_left * num_right));
                }
            }
            TokenType::Slash => {
                if non_numeric {
                    return Self::error(operator, "Operands must be two numbers for '/'");
                }
                return Ok(Value::Float(Self::as_number(operator, &left_value)? / Self::as_number(operator, &right_value)?));
            }
            TokenType::Greater => {
                let (num_left, num_right) = (Self::as_number(operator, &left_value)?, Self::as_number(operator, &right_value)?);
                return Ok(Value::Bool(num_left > num_right));
            }
            TokenType::GreaterEqual => {
                let (num_left, num_right) = (Self::as_number(operator, &left_value)?, Self::as_number(operator, &right_value)?);
                return Ok(Value::Bool(num_left >= num_right));
            }
            TokenType::Less => {
                let (num_left, num_right) = (Self::as_number(operator, &left_value)?, Self::as_number(operator, &right_value)?);
                return Ok(Value::Bool(num_left < num_right));
            }
            TokenType::LessEqual => {
                let (num_left, num_right) = (Self::as_number(operator, &left_value)?, Self::as_number(operator, &right_value)?);
                return Ok(Value::Bool(num_left <= num_right));
            }
            TokenType::EqualEqual => return Ok(Value::Bool(is_equal(&left_value, &right_value))),
            TokenType::BangEqual => return Ok(Value::Bool(!is_equal(&left_value, &right_value))),
            _ => Self::error(operator, &format!("Unsupported binary operator: {:?}", operator.token_type)),
        }
    }

    fn visit_literal(&mut self, value: &Token) -> Output {
        // Convert the token's literal to a Value
        let v = match value.literal.as_ref() {
            Some(Literal::Number(n)) => {
                // Distinguish integer vs float based on presence of decimal point in lexeme
                if value.lexeme.contains('.') {
                    Value::Float(*n)
                } else {
                    Value::Integer(*n as isize)
                }
            },
            Some(Literal::String(s)) => Value::Str(s.clone()),
            Some(Literal::Boolean(b)) => Value::Bool(*b),
            Some(Literal::Nil) => Value::Nil,
            None => Value::Nil,
        };
        return Ok(v);
    }

    // Evaluate the inner expression
    fn visit_grouping(&mut self, expression: &Expr) -> Output {
        self.evaluate(expression)
    }

    fn visit_unary(&mut self, operator: &Token, right: &Expr) -> Output {
        // Evaluate the right-hand side expression
        let right_value = self.evaluate(right)?;

        // Apply the unary operator
        match operator.token_type {
            TokenType::Minus => {
                // Return the negated number or error if it's not a number
                if let Value::Float(num) = right_value {
                    return Ok(Value::Float(-num));
                } else if let Value::Integer(num) = right_value {
                    return Ok(Value::Integer(-num));
                } else {
                    return Self::error(operator, "Operand must be a number for unary '-'");
                }
            }
            // Return the logical NOT of the truthiness of the right-hand side
            TokenType::Bang => Ok(Value::Bool(!Self::is_truthy(&right_value))),
            _ => Self::error(operator, &format!("Unsupported unary operator: {:?}", operator.token_type)),
        }
    }
}

fn is_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Float(x), Value::Float(y)) => x == y,
        (Value::Integer(x), Value::Integer(y)) => x == y,
        (Value::Str(x), Value::Str(y)) => x == y,
        // No cross-type equality in Lox
        _ => false,
    }
}