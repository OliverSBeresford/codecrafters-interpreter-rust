use std::env;
use std::fs;
use std::io::{self, Write};

mod scanner;
mod token;
mod ast;
mod parse;
mod ast_printer;

use scanner::scan;
use ast_printer::ExprNode;
use parse::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        writeln!(io::stderr(), "Usage: {} tokenize <filename>", args[0]).unwrap();
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = match fs::read_to_string(filename) {
        Ok(file_string) => file_string,
        Err(error_message) => {
            eprintln!("Failed to read file {}: {}", filename, error_message);
            std::process::exit(1);
        }
    };

    match command.as_str() {
        "tokenize" => {
            if file_contents.is_empty() {
                println!("EOF  null");
                return;
            }

            let tokens = scan(&file_contents);
            // Tokenize the input and print the tokens, regardless of lexical errors
            if let Ok(tokens) = tokens {
                print!("{}", tokens);
                return;
            } else if let Err(tokens) = tokens {
                print!("{}", tokens);
                std::process::exit(65);
            }
        }
        "parse" => {
            // Get tokens from the scanner
            let tokens = scan(&file_contents).unwrap_or_else(|_| {
                std::process::exit(65);
            });
            
            // Create a parser and parse the tokens into an AST
            let mut parser = Parser::new(&tokens.tokens);
            let expression = parser.expression();

            // Print the AST using the visit method
            match expression {
                Some(expr) => {
                    println!("{}", expr.print());
                }
                None => {
                    std::process::exit(65);
                }
            }
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
