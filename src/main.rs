use std::env;
use std::fs;
use std::io::{self, Write};
use std::iter::Peekable;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = match fs::read_to_string(filename) {
                Ok(file_string) => file_string,
                Err(error_message) => {
                    eprintln!("Failed to read file {}: {}", filename, error_message);
                    std::process::exit(1);
                }
            };

            if file_contents.is_empty() {
                println!("EOF  null");
                return;
            }

            let mut lexical_error = false;
            let mut line: usize = 1;
            let mut chars: Peekable<std::str::Chars<'_>> = file_contents.chars().peekable();

            while let Some(c) = chars.next() {
                match c {
                    // Multi-char token: ==
                    '=' => {
                        if chars.peek() == Some(&'=') {
                            chars.next();
                            println!("EQUAL_EQUAL == null");
                        } else {
                            println!("EQUAL = null");
                        }
                    },
                    '(' => println!("LEFT_PAREN ( null"),
                    ')' => println!("RIGHT_PAREN ) null"),
                    '{' => println!("LEFT_BRACE {{ null"),
                    '}' => println!("RIGHT_BRACE }} null"),
                    ',' => println!("COMMA , null"),
                    '.' => println!("DOT . null"),
                    '-' => println!("MINUS - null"),
                    '+' => println!("PLUS + null"),
                    ';' => println!("SEMICOLON ; null"),
                    '*' => println!("STAR * null"),
                    '/' => println!("SLASH / null"),

                    // whitespace & newlines
                    '\n' => {
                        line += 1;
                    }
                    c if c.is_whitespace() => { /* skip other whitespace */ }

                    '$' | '#' => {
                        eprintln!("[line {}] ERROR: Unexpected character: {}", line, c);
                        lexical_error = true;
                    }

                    _ => { /* ignore other characters for now */ }
                }
            }

            println!("EOF  null");

            if lexical_error {
                std::process::exit(65);
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
