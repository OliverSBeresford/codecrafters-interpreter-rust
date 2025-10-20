use std::env;
use std::fs;
use std::io::{self, Write};

mod scanner;
mod token;
use scanner::Scanner;

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

            let mut scanner = Scanner::new(&file_contents);
            scanner.scan_tokens();
            let lexical_error = scanner.had_error();

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
