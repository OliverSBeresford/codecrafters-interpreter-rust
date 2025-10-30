use crate::ast::{Expr, ExprVisitor};
use crate::token::{Literal, Token, TokenType};
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
    fn error(token: &Token, message: &str) -> Option<Value> {
        if token.token_type == TokenType::Eof {
            eprintln!("[line {}] Error at end: {}", token.line, message);
        } else {
            eprintln!("[line {}] Error at '{}': {}", token.line, token.lexeme, message);
        }
        return None;
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

    fn as_number(v: &Value) -> Option<f64> {
        match v {
            Value::Float(n) => Some(*n),
            Value::Integer(i) => Some(*i as f64),
            _ => None,
        }
    }

    pub fn evaluate(&mut self, expression: &Expr) -> Option<Value> {
        return expression.visit(self);
    }
}

impl<'a> ExprVisitor<'a> for Interpreter {
    type Output = Option<Value>;

    fn visit_binary(&mut self, left: &'a Expr<'a>, operator: &'a Token<'a>, right: &'a Expr<'a>) -> Self::Output {
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
                    return Some(Value::Str(format!("{}{}", str_left, str_right)));
                }
                // Handle numeric addition
                else if either_floating {
                    return Some(Value::Float(Self::as_number(&left_value)? + Self::as_number(&right_value)?));
                } else {
                    let (Value::Integer(num_left), Value::Integer(num_right)) = (left_value, right_value) else {
                        return Self::error(operator, "Operands must be two numbers or two strings for '+'");
                    };
                    return Some(Value::Integer(num_left + num_right));
                }
            }
            TokenType::Minus => {
                if non_numeric {
                    return Self::error(operator, "Operands must be two numbers for '-'");
                } else if either_floating {
                    return Some(Value::Float(Self::as_number(&left_value)? - Self::as_number(&right_value)?));
                } else {
                    let (Value::Integer(num_left), Value::Integer(num_right)) = (left_value, right_value) else {
                        return Self::error(operator, "Operands must be two integers for '-'");
                    };
                    return Some(Value::Integer(num_left - num_right));
                }
            }
            TokenType::Star => {
                if non_numeric {
                    return Self::error(operator, "Operands must be two numbers for '*'");
                } else if either_floating {
                    return Some(Value::Float(Self::as_number(&left_value)? * Self::as_number(&right_value)?));
                } else {
                    let (Value::Integer(num_left), Value::Integer(num_right)) = (left_value, right_value) else {
                        return Self::error(operator, "Operands must be two integers for '*'");
                    };
                    return Some(Value::Integer(num_left * num_right));
                }
            }
            TokenType::Slash => {
                if non_numeric {
                    return Self::error(operator, "Operands must be two numbers for '/'");
                }
                return Some(Value::Float(Self::as_number(&left_value)? / Self::as_number(&right_value)?));
            }
            TokenType::Greater => {
                let (num_left, num_right) = (Self::as_number(&left_value), Self::as_number(&right_value));
                match (num_left, num_right) {
                    (Some(num_left), Some(num_right)) => Some(Value::Bool(num_left > num_right)),
                    _ => Self::error(operator, "Operands must be numbers for '>'"),
                }
            }
            TokenType::GreaterEqual => {
                let (num_left, num_right) = (Self::as_number(&left_value), Self::as_number(&right_value));
                match (num_left, num_right) {
                    (Some(num_left), Some(num_right)) => Some(Value::Bool(num_left >= num_right)),
                    _ => Self::error(operator, "Operands must be numbers for '>='"),
                }
            }
            TokenType::Less => {
                let (num_left, num_right) = (Self::as_number(&left_value), Self::as_number(&right_value));
                match (num_left, num_right) {
                    (Some(num_left), Some(num_right)) => Some(Value::Bool(num_left < num_right)),
                    _ => Self::error(operator, "Operands must be numbers for '<'"),
                }
            }
            TokenType::LessEqual => {
                let (num_left, num_right) = (Self::as_number(&left_value), Self::as_number(&right_value));
                match (num_left, num_right) {
                    (Some(num_left), Some(num_right)) => Some(Value::Bool(num_left <= num_right)),
                    _ => Self::error(operator, "Operands must be numbers for '<='"),
                }
            }
            TokenType::EqualEqual => Some(Value::Bool(Self::is_equal(&left_value, &right_value))),
            TokenType::BangEqual => Some(Value::Bool(!Self::is_equal(&left_value, &right_value))),
            _ => Self::error(operator, &format!("Unsupported binary operator: {:?}", operator.token_type)),
        }
    }

    fn visit_literal(&mut self, value: &'a Token<'a>) -> Self::Output {
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
        return Some(v);
    }

    // Start the visitor pattern on the inner expression
    fn visit_grouping(&mut self, expression: &'a Expr<'a>) -> Self::Output {
        expression.visit(self)
    }

    fn visit_unary(&mut self, operator: &'a Token<'a>, right: &'a Expr<'a>) -> Self::Output {
        // Evaluate the right-hand side expression
        let right_value = self.evaluate(right)?;

        // Apply the unary operator
        match operator.token_type {
            TokenType::Minus => {
                // Return the negated number or error if it's not a number
                if let Value::Float(num) = right_value {
                    return Some(Value::Float(-num));
                } else if let Value::Integer(num) = right_value {
                    return Some(Value::Integer(-num));
                } else {
                    return Self::error(operator, "Operand must be a number");
                }
            }
            // Return the logical NOT of the truthiness of the right-hand side
            TokenType::Bang => Some(Value::Bool(!Self::is_truthy(&right_value))),
            _ => Self::error(operator, &format!("Unsupported unary operator: {:?}", operator.token_type)),
        }
    }
}
