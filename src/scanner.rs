use std::iter::Peekable;
use std::str::CharIndices;

use crate::token::Token;
use crate::token::TokenType;
use crate::token::Literal;

pub struct Scanner<'a> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
    line: usize,
    start: usize,
    current: usize,
    lexical_error: bool,
    tokens: Vec<Token<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
            line: 1,
            start: 0,
            current: 0,
            lexical_error: false,
            tokens: Vec::new(),
        }
    }

    // Start a token
    fn begin_token(&mut self) {
        self.start = self.current;
    }

    // Advance the scanner by one character and return it
    fn advance(&mut self) -> Option<char> {
        if let Some((byte_index, ch)) = self.chars.next() {
            self.current = byte_index + ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }

    // Create a new token and add it to the tokens vector
    fn make_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme = &self.input[self.start..self.current];
        let token = Token::new(token_type, lexeme, literal, self.line);
        println!("{}", &token);
        self.tokens.push(token);
    }

    pub fn scan_tokens(&mut self) {
        while self.peek().is_some() {
            self.scan_token();
        }
    }

    fn scan_token(&mut self) {
        self.begin_token();
        if let Some(c) = self.advance() {
            match c {
                // Multi-char tokens
                '=' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.make_token(TokenType::EqualEqual, None);
                    } else {
                        self.make_token(TokenType::Equal, None);
                    }
                }
                '!' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.make_token(TokenType::BangEqual, None);
                    } else {
                        self.make_token(TokenType::Bang, None);
                    }
                }
                '<' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.make_token(TokenType::LessEqual, None);
                    } else {
                        self.make_token(TokenType::Less, None);
                    }
                }
                '>' => {
                    if self.peek() == Some('=') {
                        self.advance();
                        self.make_token(TokenType::GreaterEqual, None);
                    } else {
                        self.make_token(TokenType::Greater, None);
                    }
                }

                // Single-char tokens
                '(' => self.make_token(TokenType::LeftParen, None),
                ')' => self.make_token(TokenType::RightParen, None),
                '{' => self.make_token(TokenType::LeftBrace, None),
                '}' => self.make_token(TokenType::RightBrace, None),
                ',' => self.make_token(TokenType::Comma, None),
                '.' => self.make_token(TokenType::Dot, None),
                '-' => self.make_token(TokenType::Minus, None),
                '+' => self.make_token(TokenType::Plus, None),
                ';' => self.make_token(TokenType::Semicolon, None),
                '*' => self.make_token(TokenType::Star, None),

                // whitespace & newlines
                '\n' => {
                    self.line += 1;
                }
                c if c.is_whitespace() => { /* skip other whitespace */ }

                // Comments and division
                '/' => {
                    if self.peek() == Some('/') {
                        // consume rest of line
                        while let Some(&(_, next_char)) = self.chars.peek() {
                            if next_char == '\n' {
                                break;
                            }
                            self.advance();
                        }
                    } else {
                        self.make_token(TokenType::Slash, None);
                    }
                }
                // string literals
                '"' => {
                    self.scan_string();
                }

                // unexpected characters
                other => {
                    eprintln!("[line {}] ERROR: Unexpected character: {}", self.line, other);
                    self.lexical_error = true;
                }
            };
        }
    }

    fn scan_string(&mut self) {
        // Consume the opening quote
        self.advance();

        while let Some(c) = self.advance() {
            if c == '"' {
                // Consume the closing quote
                let string_literal = &self.input[self.start + 1..self.current - 1];
                self.make_token(TokenType::String, Some(Literal::String(string_literal.to_string())));
                return;
            }
        }

        // If we reach the end of the input without finding a closing quote, it's an error
        eprintln!("[line {}] ERROR: Unterminated string literal", self.line);
        self.lexical_error = true;
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, ch)| ch)
    }

    pub fn had_error(&self) -> bool {
        self.lexical_error
    }
}