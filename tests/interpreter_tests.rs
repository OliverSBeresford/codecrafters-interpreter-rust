use rust_interpreter::{Interpreter, Parser, Value, scan};
use rust_interpreter::runtime::{Environment, EnvRef, Function, Callable};
use rust_interpreter::Expr;
use rust_interpreter::ast::Statement;

fn parse_expr(input: &str) -> (Interpreter, Expr) {
    let tokens = scan(input);
    let mut parser = Parser::new(tokens.tokens);
    let expr = parser.expression().unwrap_or_else(|e| panic!("parse error: {}", e));
    (Interpreter::new(), expr)
}

fn parse_stmts(input: &str) -> (Interpreter, Vec<Statement>) {
    let tokens = scan(input);
    let mut parser = Parser::new(tokens.tokens);
    let statements = parser.parse();
    (Interpreter::new(), statements)
}

#[test]
fn evaluate_addition() {
    let (mut interpreter, expr) = parse_expr("1 + 2");
    let v = interpreter.evaluate(&expr).unwrap_or_else(|_| panic!("eval error"));
    match v {
        Value::Integer(n) => assert_eq!(n, 3),
        other => panic!("unexpected value: {:?}", other),
    }
}

#[test]
fn evaluate_unary_minus() {
    let (mut interpreter, expr) = parse_expr("-5");
    let v = interpreter.evaluate(&expr).unwrap_or_else(|_| panic!("eval error"));
    match v {
        Value::Integer(n) => assert_eq!(n, -5),
        other => panic!("unexpected value: {:?}", other),
    }
}

#[test]
fn evaluate_logic_not_truthiness() {
    let (mut interpreter, expr) = parse_expr("!123");
    let v = interpreter.evaluate(&expr).unwrap_or_else(|_| panic!("eval error"));
    match v {
        Value::Bool(b) => assert_eq!(b, false),
        other => panic!("unexpected value: {:?}", other),
    }
}

#[test]
fn evaluate_variable_lookup() {
    let (mut interpreter, expr) = parse_expr("a");
    // define variable in environment
    interpreter.environment.borrow_mut().define("a".to_string(), Value::Integer(42));
    let v = interpreter.evaluate(&expr).unwrap_or_else(|_| panic!("eval error"));
    match v {
        Value::Integer(n) => assert_eq!(n, 42),
        other => panic!("unexpected value: {:?}", other),
    }
}

#[test]
fn function_call_returns_sum() {
    // Parse a function declaration
    let (mut interpreter, statements) = parse_stmts(
        "
        fun add(x, y) {
            return x + y;
        }
        ",
    );
    // Make sure we have only one statement which is the function declaration
    assert!(statements.len() == 1, "expected one statement");
    let stmt = statements.into_iter().next().expect("one statement expected");
    
    // bind in current environment
    let env: EnvRef = Environment::new(None);
    interpreter.environment = env.clone();

    // Build function from statement
    let func = Function::from_statement(&stmt, env.clone()).unwrap_or_else(|_| panic!("function build error"));
    
    // Call the function with args
    let result = func
        .call(&mut interpreter, vec![Value::Integer(2), Value::Integer(3)])
        .unwrap_or_else(|_| panic!("call failed"));
    match result {
        Value::Integer(n) => assert_eq!(n, 5),
        other => panic!("unexpected value: {:?}", other),
    }
}
