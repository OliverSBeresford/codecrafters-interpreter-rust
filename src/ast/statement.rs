use std::rc::Rc;

use crate::ast::expr::Expr;
use crate::lexer::token::Token;

pub type StatementRef = Rc<Statement>;

#[derive(Debug)]
pub enum Statement {
    Expression {
        expression: Expr,
    },
    If {
        condition: Expr,
        then_branch: StatementRef,
        else_branch: Option<StatementRef>,
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
        body: StatementRef,
    },
    Block {
        statements: Vec<StatementRef>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<StatementRef>,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
}
