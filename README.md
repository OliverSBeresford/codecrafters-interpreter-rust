# A tiny Lox interpreter in Rust

This repository contains an in-progress Rust implementation of Lox from Crafting Interpreters. It includes a scanner, parser that builds an AST, and a tree-walk interpreter with environments, functions, and control flow.

## Features

- Tokenize Lox source into a stream of tokens
- Parse expressions and statements into an AST
- Print a parenthesized representation of the AST for debugging
- Evaluate expressions and print results
- Execute variable declarations, blocks, `if`/`else`, `while`, `for`, `print`, and function calls

## Requirements

- Rust stable toolchain (https://rustup.rs)
- A Unix-like shell to run `your_program.sh`

## Quickstart

Build once, then run the helper script with a command and input file:

```sh
./your_program.sh <command> test.lox
```

Common commands:

```sh
# Print tokens
./your_program.sh tokenize test.lox

# Print the AST in parenthesized form
./your_program.sh parse test.lox

# Evaluate a single expression
./your_program.sh evaluate test.lox

# Run a program consisting of statements
./your_program.sh run test.lox

# Dump tokens and parsed statements for debugging
./your_program.sh dbg test.lox
```

## Example

Given `test.lox`:

```lox
7 * 3 / 7 / 1
```

Tokens:

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

AST (parenthesized):

```text
(/ (/ (* 7.0 3.0) 7.0) 1.0)
```

Evaluation result:

```text
3
```

## Development

- Run the full test suite: `cargo test`
- Run only lexer/parser/interpreter tests: `cargo test lexer_tests`, `cargo test parser_tests`, `cargo test interpreter_tests`

## References

- Language design: https://craftinginterpreters.com/the-lox-language.html
- Book: Crafting Interpreters by Robert Nystrom: https://craftinginterpreters.com/
