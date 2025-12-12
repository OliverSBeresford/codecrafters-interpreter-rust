use crate::token::Token;
use crate::statement_syntax_tree::Statement;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    LogicOr {
        left: Box<Expr>,
        // operator: Token, Right now we don't use the operator token, but it's here for completeness
        right: Box<Expr>,
    },
    LogicAnd {
        left: Box<Expr>,
        // operator: Token, Right now we don't use the operator token, but it's here for completeness
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal {
        value: Token,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Variable {
        name: Token,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Lambda {
        params: Vec<Token>,
        body: Vec<Rc<Statement>>,
    },
}
