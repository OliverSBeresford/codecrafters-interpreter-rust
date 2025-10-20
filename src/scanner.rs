use std::iter::Peekable;
use std::str::Chars;

use crate::token::Token;
use crate::token::TokenType;
use crate::token::Literal;

pub struct Scanner<'a> {
    chars: Peekable<Chars<'a>>,
    line: usize,
    lexical_error: bool,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            line: 1,
            lexical_error: false,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) {
        while self.peek().is_some() {
            self.scan_token();
        }
    }

    fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            let new_token: Option<Token> = match c {
                // Multi-char tokens
                '=' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        Some(Token::new(TokenType::EqualEqual, "==".to_string(), None, self.line))
                    } else {
                        Some(Token::new(TokenType::Equal, "=".to_string(), None, self.line))
                    }
                }
                '!' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        Some(Token::new(TokenType::BangEqual, "!=".to_string(), None, self.line))
                    } else {
                        Some(Token::new(TokenType::Bang, "!".to_string(), None, self.line))
                    }
                }
                '<' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        Some(Token::new(TokenType::LessEqual, "<=".to_string(), None, self.line))
                    } else {
                        Some(Token::new(TokenType::Less, "<".to_string(), None, self.line))
                    }
                }
                '>' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        Some(Token::new(TokenType::GreaterEqual, ">=".to_string(), None, self.line))
                    } else {
                        Some(Token::new(TokenType::Greater, ">".to_string(), None, self.line))
                    }
                }

                // Single-char tokens
                '(' => Some(Token::new(TokenType::LeftParen, "(".to_string(), None, self.line)),
                ')' => Some(Token::new(TokenType::RightParen, ")".to_string(), None, self.line)),
                '{' => Some(Token::new(TokenType::LeftBrace, "{".to_string(), None, self.line)),
                '}' => Some(Token::new(TokenType::RightBrace, "}".to_string(), None, self.line)),
                ',' => Some(Token::new(TokenType::Comma, ",".to_string(), None, self.line)),
                '.' => Some(Token::new(TokenType::Dot, ".".to_string(), None, self.line)),
                '-' => Some(Token::new(TokenType::Minus, "-".to_string(), None, self.line)),
                '+' => Some(Token::new(TokenType::Plus, "+".to_string(), None, self.line)),
                ';' => Some(Token::new(TokenType::Semicolon, ";".to_string(), None, self.line)),
                '*' => Some(Token::new(TokenType::Star, "*".to_string(), None, self.line)),

                // whitespace & newlines
                '\n' => {
                    self.line += 1;
                    None
                }
                c if c.is_whitespace() => { None /* skip other whitespace */ }

                // Comments and division
                '/' => {
                    if self.peek() == Some('/') {
                        // consume rest of line
                        while let Some(&next_char) = self.chars.peek() {
                            if next_char == '\n' {
                                break;
                            }
                            self.advance();
                        }
                        None
                    } else {
                        Some(Token::new(TokenType::Slash, "/".to_string(), None, self.line))
                    }
                }
                // string literals
                '"' => {
                    self.scan_string()
                }

                // unexpected characters
                other => {
                    eprintln!("[line {}] ERROR: Unexpected character: {}", self.line, other);
                    self.lexical_error = true;
                    None
                }
            };
            // Add the new token if one was created
            if let Some(token) = new_token {
                println!("{}", &token);
                self.tokens.push(token);
            }
        }
    }

    fn scan_string(&mut self) -> Option<Token> {
        // Implementation for scanning string literals would go here
        let mut string_literal = String::new();

        // Consume the opening quote
        self.advance();

        while let Some(c) = self.advance() {
            if c == '"' {
                // Consume the closing quote
                return Some(Token::new(TokenType::String, string_literal.clone(), Some(Literal::String(string_literal)), self.line));
            }
            string_literal.push(c);
        }

        // If we reach the end of the input without finding a closing quote, it's an error
        eprintln!("[line {}] ERROR: Unterminated string literal", self.line);
        self.lexical_error = true;
        None
    }

    fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn had_error(&self) -> bool {
        self.lexical_error
    }
}