use std::thread::current;

use crate::token::Token;
use crate::token::TokenType;
use crate::token::Literal;
use crate::ast::Expr;
use crate::token::Keyword::{Nil, False, True};

pub struct Parser<'a> {
    tokens: &'a Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    // Advance the parser by one token and return it (with ownership)
    fn advance(&mut self) -> Option<Token<'a>> {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            return Some(token);
        } else {
            return None;
        }
    }

    // Return the next token without consuming it
    fn peek(&self) -> Option<&Token<'a>> {
        return self.tokens.get(self.current + 1);
    }

    // Check if the next token is of any of the expected types
    fn check(&self, expected: &[TokenType]) -> bool {
        if let Some(token) = self.peek() {
            return expected.contains(&token.token_type);
        }
        return false;
    }

    // Return current token
    fn current_token(&self) -> Option<&Token<'a>> {
        return self.tokens.get(self.current);
    }

    // Consume a token of the expected type, or return an error
    fn consume(&mut self, expected: TokenType, error_message: &str) -> Option<Token<'a>> {
        let current_token = self.advance();

        // If the current token is not of the expected type or doesn't exist, return an error
        if current_token.is_none() || current_token.as_ref().unwrap().token_type != expected {
            println!("Parse error: {}", error_message);
            return None;
        }

        return current_token;
    }

    fn expression(&mut self) -> Option<Expr<'a>> {
        return self.equality();
    }

    // Lowest precedence, going up from here
    fn equality(&mut self) -> Option<Expr<'a>> {
        // Create the left-hand side expression
        let mut expr = self.comparison().unwrap();

        while self.check(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            // Consume the operator and store it
            let operator = self.advance().unwrap();
            let right = self.comparison().unwrap();

            // Create a new binary expression with the left and right expressions
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Some(expr);
    }

    // A comparison is a term followed by zero or more <, >, <=, >=, each followed by a term, like 1 < 2 >= 3
    fn comparison(&mut self) -> Option<Expr<'a>> {
        // Create the left-hand side expression (can be a term or above)
        let mut expr = self.term().unwrap();

        while self.check(&[TokenType::Less, TokenType::Greater, TokenType::LessEqual, TokenType::GreaterEqual]) {
            // Consume the operator and store it
            let operator = self.advance().unwrap();
            let right = self.term().unwrap();

            // Create a new binary expression with the left and right expressions
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Some(expr);
    }

    // A term is a factor followed by zero or more + or -, each followed by a factor, like 1 + 2 - 3
    fn term(&mut self) -> Option<Expr<'a>> {
        // Create the left-hand side expression (can be a factor or above)
        let mut expr = self.factor().unwrap();

        while self.check(&[TokenType::Minus, TokenType::Plus]) {
            // Consume the operator and store it
            let operator = self.advance().unwrap();
            let right = self.factor().unwrap();

            // Create a new binary expression with the left and right expressions
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Some(expr);
    }

    // A factor is a unary expression followed by zero or more * or /, each followed by a unary expression, like -4 / 2 * 3
    fn factor(&mut self) -> Option<Expr<'a>> {
        // Create the left-hand side expression (can be a unary or above)
        let mut expr = self.unary().unwrap();

        while self.check(&[TokenType::Slash, TokenType::Star]) {
            // Consume the operator and store it
            let operator = self.advance().unwrap();
            let right = self.unary().unwrap();

            // Create a new binary expression with the left and right expressions
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Some(expr);
    }

    // A unary expression is either a primary expression or a unary operator followed by another unary expression, like -!!5
    fn unary(&mut self) -> Option<Expr<'a>> {
        if self.check(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.advance().unwrap();
            let right = self.unary().unwrap();

            return Some(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        return self.primary();
    }

    // A primary expression is either a literal value or a parenthesized expression
    fn primary(&mut self) -> Option<Expr<'a>> {
        let current_token = self.advance().unwrap();

        match current_token.token_type {
            TokenType::Number | TokenType::String => {
                return Some(Expr::Literal{
                    value: current_token
                });
            },
            TokenType::LeftParen => {
                let expr = self.expression();
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                return Some(Expr::Grouping{
                    expression: Box::new(expr.unwrap())
                });
            }
            TokenType::Keyword(Nil) | TokenType::Keyword(False) | TokenType::Keyword(True) => {
                return Some(Expr::Literal{
                    value: current_token
                });
            }
            _ => {
                println!("Parse error: Unexpected token {:?}", current_token);
                return None;
            }
        }
    }
}