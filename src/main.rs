use std::env;
use std::fs;
use std::io::{self, Write};

mod scanner;
mod token;
mod expr_syntax_tree;
mod parse;
mod ast_printer;
mod interpreter;
mod runtime_error;
mod statement_syntax_tree;
mod parse_error;
mod environment;
mod value;

use scanner::scan;
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

            // Tokenize the input and print the tokens
            print!("{}", tokens); 
        }
        "parse" => {
            // Get tokens from the scanner
            let tokens = scan(&file_contents);
            
            // Create a parser and parse the tokens into an AST
            let mut parser = Parser::new(&tokens.tokens);
            let expression = parser.expression();

            // Print the AST using the visit method
            match expression {
                Ok(expr) => {
                    ast_printer::AstPrinter.print(&expr);
                }
                Err(error) => {
                    eprintln!("{}", error);
                    std::process::exit(65);
                }
            }
        }
        "evaluate" => {
            // Get tokens from the scanner
            let tokens = scan(&file_contents);
            
            // Create a parser and parse the tokens into an AST
            let mut parser = Parser::new(&tokens.tokens);
            let expression = parser.expression().unwrap_or_else(|error| {
                eprintln!("{}", error);
                std::process::exit(65);
            });

            // Create an interpreter and evaluate the expression
            let mut interpreter = interpreter::Interpreter::new();
            let result = interpreter.evaluate(&expression).unwrap_or_else(|runtime_error| {
                eprintln!("{}", runtime_error);
                std::process::exit(70);
            });
            
            // Print the result of the evaluation
            println!("{}", result);
        }
        "run" => {
            // Get tokens from the scanner
            let tokens = scan(&file_contents);
            
            // Create a parser and parse the tokens into statements
            let mut parser = Parser::new(&tokens.tokens);
            let statements = parser.parse();

            // Create an interpreter and execute the statements
            let mut interpreter = interpreter::Interpreter::new();
            interpreter.interpret(statements);
        }
        "dbg" => {
            // Create an AST printer
            let ast_printer = ast_printer::AstPrinter;

            // Get tokens from the scanner
            let tokens = scan(&file_contents);
            println!("Tokens:\n{}\n", tokens);
            
            // Create a parser and parse the tokens into statements
            let mut parser = Parser::new(&tokens.tokens);
            let statements = parser.parse();

            // Print the AST of the statements
            println!("AST of the statements:");
            ast_printer.print_statements(&statements);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
