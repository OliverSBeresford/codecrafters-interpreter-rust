use crate::token::Token;
use crate::token::TokenType;
use crate::expr_syntax_tree::Expr;
use crate::token::Keyword::{Nil, False, True};
use crate::token::Keyword;
use crate::statement_syntax_tree::Statement;
use crate::parse_error::ParseError;

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

    // Report a parse error
    fn error<T>(token: &Token<'a>, message: &str) -> Result<T, ParseError> {
        if token.token_type == TokenType::Eof {
            return Err(ParseError::new(token.line, format!("Error at end: {}", message)));
        } else {
            return Err(ParseError::new(token.line, format!("Error at '{}': {}", token.lexeme, message)));
        }
    }

    // A synchronization method to recover from errors
    fn synchronize(&mut self) {
        self.consume_any();

        while let Some(token) = self.current_token() {
            if token.token_type == TokenType::Semicolon {
                self.consume_any();
                return;
            }

            match token.token_type {
                TokenType::Keyword(kw) => match kw {
                    Keyword::Class | Keyword::Fun | Keyword::Var | Keyword::For | Keyword::If | Keyword::While | Keyword::Print | Keyword::Return => {
                        return;
                    }
                    _ => {}
                },
                _ => {}
            }

            self.consume_any();
        }
    }

    // Return the current token and advance the parser
    fn advance(&mut self) -> Result<Token<'a>, ParseError> {
        if self.current < self.tokens.len() {
            let token = self.tokens[self.current].clone();
            self.current += 1;
            return Ok(token);
        } else {
            return Self::error(&self.tokens[self.tokens.len() - 1], "Unexpected end of input");
        }
    }

    // Get the current token without advancing the parser
    fn current_token(&self) -> Option<&Token<'a>> {
        return self.tokens.get(self.current);
    }

    // Check if the current token is of one of the expected types
    fn check(&self, expected: &[TokenType]) -> bool {
        if let Some(token) = self.current_token() {
            return expected.contains(&token.token_type);
        }
        return false;
    }

    // Consume a token of the expected type, or return an error
    fn consume(&mut self, expected: TokenType, error_message: &str) -> Result<Token<'a>, ParseError> {
        let current_token = self.advance()?;

        // If the current token is not of the expected type or doesn't exist, return an error
        if current_token.token_type != expected {
            return Self::error(&current_token, error_message);
        }

        return Ok(current_token);
    }

    fn consume_any(&mut self) {
        let _ = self.advance();
    }

    pub fn parse(&mut self) -> Vec<Statement<'a>> {
        let mut statements: Vec<Statement<'a>> = Vec::new();

        // Parse statements until the end of the token stream (-1 for EOF)
        while self.current < self.tokens.len() - 1 {
            let statement = self.declaration();
            if let Err(e) = &statement {
                eprintln!("{}", e);
            }
            else if let Ok(statement) = statement {
                statements.push(statement);
            }
        }
        return statements;
    }

    fn declaration(&mut self) -> Result<Statement<'a>, ParseError> {
        // For now, only parse variable declarations and statements
        if self.check(&[TokenType::Keyword(Keyword::Var)]) {
            return self.var_declaration().or_else(|err: ParseError| {
                self.synchronize(); // Synchronize on error
                Err(err)
            });
        }
        return self.statement().or_else(|err: ParseError| {
            self.synchronize(); // Synchronize on error
            Err(err)
        });
    }

    fn var_declaration(&mut self) -> Result<Statement<'a>, ParseError> {
        // Consume the 'var' keyword
        let _var_token = self.advance();

        // Consume the variable name
        let name_token = self.consume(TokenType::Identifier, "Expect variable name.")?;

        // Optional initializer
        let initializer = if self.check(&[TokenType::Equal]) {
            // Consume the '=' token
            let _equal_token = self.advance();

            // Parse the initializer expression
            Some(self.expression()?)
        } else {
            None
        };

        // Consume the semicolon
        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;

        return Ok(Statement::Var {
            name: name_token,
            initializer,
        });
    }

    fn statement(&mut self) -> Result<Statement<'a>, ParseError> {
        // For now, only parse expression and print statements
        if self.check(&[TokenType::Keyword(Keyword::Print)]) {
            return self.print_statement();
        } else if self.check(&[TokenType::LeftBrace]) {
            return self.block_statement();
        } else if self.check(&[TokenType::Keyword(Keyword::If)]) {
            return self.if_statement();
        } else {
            return self.expression_statement();
        }
    }

    fn print_statement(&mut self) -> Result<Statement<'a>, ParseError> {
        // Consume the 'print' keyword
        let _print_token = self.advance();

        // Parse the expression to be printed
        let expression = self.expression()?;

        // Consume the semicolon at the end of the print statement
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;

        return Ok(Statement::Print {
            expression,
        });
    }

    fn expression_statement(&mut self) -> Result<Statement<'a>, ParseError> {
        let expression = self.expression()?;

        // Consume the semicolon at the end of the expression statement
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;

        return Ok(Statement::Expression {
            expression,
        });
    }

    fn block_statement(&mut self) -> Result<Statement<'a>, ParseError> {
        // Consume the '{' token
        let _left_brace = self.advance();

        // Create a vector to hold the statements in the block
        let mut statements: Vec<Statement<'a>> = Vec::new();

        // Parse statements until we find a '}'
        while !self.check(&[TokenType::RightBrace]) && self.current < self.tokens.len() - 1 {
            let declaration = self.declaration()?;
            statements.push(declaration);
        }

        // Consume the '}' token
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        return Ok(Statement::Block {
            statements,
        });
    }

    fn if_statement(&mut self) -> Result<Statement<'a>, ParseError> {
        // Consume the 'if' keyword
        let _if_token = self.advance();

        // Parse the condition expression and consume the parentheses
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        // Parse the then branch statement
        let then_branch = Box::new(self.statement()?);

        // Optional else branch
        let else_branch = if self.check(&[TokenType::Keyword(Keyword::Else)]) {
            // Consume the 'else' keyword
            let _else_token = self.advance();

            // Parse the else branch statement
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        return Ok(Statement::IfStatement {
            condition,
            then_branch,
            else_branch,
        });
    }

    pub fn expression(&mut self) -> Result<Expr<'a>, ParseError> {
        return self.assignment();
    }

    fn assignment(&mut self) -> Result<Expr<'a>, ParseError> {
        let expr = self.logic_or()?;

        if self.check(&[TokenType::Equal]) {
            let equals = self.advance()?;
            let value = self.assignment()?;
            
            // If the left-hand side is a variable, create an assignment expression
            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }

            return Self::error(&equals, "Invalid assignment target.");
        }

        return Ok(expr);
    }

    fn logic_or(&mut self) -> Result<Expr<'a>, ParseError> {
        let mut expr = self.logic_and()?;

        while self.check(&[TokenType::Keyword(Keyword::Or)]) {
            let _operator = self.advance()?;
            let right = self.logic_and()?;

            expr = Expr::LogicOr {
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        return Ok(expr);
    }

    fn logic_and(&mut self) -> Result<Expr<'a>, ParseError> {
        let mut expr = self.equality()?;

        while self.check(&[TokenType::Keyword(Keyword::And)]) {
            let _operator = self.advance()?;
            let right = self.equality()?;

            expr = Expr::LogicAnd {
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        return Ok(expr);
    }

    // Lowest precedence, going up from here
    fn equality(&mut self) -> Result<Expr<'a>, ParseError> {
        // Create the left-hand side expression
        let mut expr = self.comparison()?;

        while self.check(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            // Consume the operator and store it
            let operator = self.advance()?;
            let right = self.comparison()?;

            // Create a new binary expression with the left and right expressions
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    // A comparison is a term followed by zero or more <, >, <=, >=, each followed by a term, like 1 < 2 >= 3
    fn comparison(&mut self) -> Result<Expr<'a>, ParseError> {
        // Create the left-hand side expression (can be a term or above)
        let mut expr = self.term()?;

        while self.check(&[TokenType::Less, TokenType::Greater, TokenType::LessEqual, TokenType::GreaterEqual]) {
            // Consume the operator and store it
            let operator = self.advance()?;
            let right = self.term()?;

            // Create a new binary expression with the left and right expressions
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    // A term is a factor followed by zero or more + or -, each followed by a factor, like 1 + 2 - 3
    fn term(&mut self) -> Result<Expr<'a>, ParseError> {
        // Create the left-hand side expression (can be a factor or above)
        let mut expr = self.factor()?;

        while self.check(&[TokenType::Minus, TokenType::Plus]) {
            // Consume the operator and store it
            let operator = self.advance()?;
            let right = self.factor()?;

            // Create a new binary expression with the left and right expressions
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    // A factor is a unary expression followed by zero or more * or /, each followed by a unary expression, like -4 / 2 * 3
    fn factor(&mut self) -> Result<Expr<'a>, ParseError> {
        // Create the left-hand side expression (can be a unary or above)
        let mut expr = self.unary()?;

        while self.check(&[TokenType::Slash, TokenType::Star]) {
            // Consume the operator and store it
            let operator = self.advance()?;
            let right = self.unary()?;

            // Create a new binary expression with the left and right expressions
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        return Ok(expr);
    }

    // A unary expression is either a primary expression or a unary operator followed by another unary expression, like -!!5
    fn unary(&mut self) -> Result<Expr<'a>, ParseError> {
        if self.check(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.advance()?;
            let right = self.unary()?;

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        return self.primary();
    }

    // A primary expression is either a literal value or a parenthesized expression
    fn primary(&mut self) -> Result<Expr<'a>, ParseError> {
        let current_token = self.advance()?;

        match current_token.token_type {
            TokenType::Number | TokenType::String => {
                return Ok(Expr::Literal{
                    value: current_token
                });
            },
            TokenType::LeftParen => {
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect expression.")?;
                return Ok(Expr::Grouping{
                    expression: Box::new(expr)
                });
            }
            TokenType::Keyword(Nil) | TokenType::Keyword(False) | TokenType::Keyword(True) => {
                return Ok(Expr::Literal{
                    value: current_token
                });
            }
            TokenType::Identifier => {
                return Ok(Expr::Variable{
                    name: current_token
                });
            }
            _ => {
                return Self::error(&current_token, "Expect expression.");
            }
        }
    }
}