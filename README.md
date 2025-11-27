# A tiny Lox interpreter in Rust

This repository contains a Rust implementation-in-progress of a small programming language inspired by Lox from the book Crafting Interpreters. The goal is to build a tokenizer, parser (producing an AST), and a tree-walk interpreter.

Current capabilities:
- Tokenize Lox source code into a stream of tokens
- Parse expressions and statements into an AST
- Print a parenthesized representation of the AST (for debugging)
- Evaluate expressions and print the result
- Execute variable declaration, if, print, expression, and block statements

## Usage

The entrypoint script compiles and runs the binary. Use it like this:

```sh
./your_program.sh <command> test.lox
```

Examples:

```sh
# Print tokens
./your_program.sh tokenize test.lox

# Print the AST in parenthesized form
./your_program.sh parse test.lox

# Evaluate the expression(s) and print the result
./your_program.sh evaluate test.lox
```

## Example

Given the following `test.lox`:

```lox
7 * 3 / 7 / 1
```

Expected outputs:

- Tokens

```text
NUMBER 7 7.0
STAR * null
NUMBER 3 3.0
SLASH / null
NUMBER 7 7.0
SLASH / null
NUMBER 1 1.0
EOF  null
```

- AST (parenthesized)

```text
(/ (/ (* 7.0 3.0) 7.0) 1.0)
```

- Evaluation result

```text
3
```

## Commands

- tokenize: Scans the input and prints tokens (one per line), including EOF. Intended for validating the scanner.
- parse: Parses the input into an expression AST and prints it in a readable parenthesized format.
- evaluate: Parses and evaluates the input program and prints the resulting value.
- run: Parses and runs the program line by line as a list of statements
- dbg: Prints out all of the tokens and an AST representation of the statements and expressions in the program

More commands will be added as the interpreter evolves.

## References

- The language design is based on Lox: https://craftinginterpreters.com/the-lox-language.html
- This project follows ideas from the excellent book Crafting Interpreters by Robert Nystrom: https://craftinginterpreters.com/
