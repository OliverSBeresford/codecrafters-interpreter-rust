use std::env;
use std::fs;
use std::io::{self, Write};

use rust_interpreter::{AstPrinter, ControlFlow, Interpreter, Parser, scan};

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
            let mut parser = Parser::new(tokens.tokens);
            let expression = parser.expression();

            // Print the AST using the visit method
            match expression {
                Ok(expr) => {
                    AstPrinter.print(&expr);
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
            let mut parser = Parser::new(tokens.tokens);
            let expression = parser.expression().unwrap_or_else(|error| {
                eprintln!("{}", error);
                std::process::exit(65);
            });

            // Create an interpreter and evaluate the expression
            let mut interpreter = Interpreter::new();
            let result = interpreter.evaluate(&expression).unwrap_or_else(|control_flow| {
                if let ControlFlow::RuntimeError(runtime_error) = control_flow {
                    eprintln!("{}", runtime_error);
                    std::process::exit(70);
                }
                std::process::exit(70);
            });
            
            // Print the result of the evaluation
            println!("{}", result);
        }
        "run" => {
            // Get tokens from the scanner
            let tokens = scan(&file_contents);
            
            // Create a parser and parse the tokens into statements
            let mut parser = Parser::new(tokens.tokens);
            let statements = parser.parse();

            // Create an interpreter and execute the statements
            let mut interpreter = Interpreter::new();
            interpreter.interpret(statements);
        }
        "dbg" => {
            // Get tokens from the scanner
            let tokens = scan(&file_contents);
            println!("Tokens:\n{}\n", tokens);
            
            // Create a parser and parse the tokens into statements
            let mut parser = Parser::new(tokens.tokens);
            let statements = parser.parse();

            // Print the AST of the statements
            dbg!("Parsed Statements AST:", &statements);
        }
        _ => {
            writeln!(io::stderr(), "Unknown command: {}", command).unwrap();
            return;
        }
    }
}
