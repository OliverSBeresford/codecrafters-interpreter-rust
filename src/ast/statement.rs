use crate::ast::expr::Expr;
use crate::lexer::token::Token;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub enum Statement {
    Expression {
        expression: Expr,
    },
    If {
        condition: Expr,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Print {
        expression: Expr,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    While {
        condition: Expr,
        body: Box<Statement>,
    },
    Block {
        statements: Vec<Statement>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Rc<RefCell<Vec<Statement>>>,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
}
