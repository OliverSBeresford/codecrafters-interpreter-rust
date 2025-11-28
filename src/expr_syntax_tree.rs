use crate::token::Token;

#[derive(Debug)]
pub enum Expr<'a> {
    Assign {
        name: Token<'a>,
        value: Box<Expr<'a>>,
    },
    LogicOr {
        left: Box<Expr<'a>>,
        // operator: Token<'a>, Right now we don't use the operator token, but it's here for completeness
        right: Box<Expr<'a>>,
    },
    LogicAnd {
        left: Box<Expr<'a>>,
        // operator: Token<'a>, Right now we don't use the operator token, but it's here for completeness
        right: Box<Expr<'a>>,
    },
    Binary {
        left: Box<Expr<'a>>,
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Literal {
        value: Token<'a>,
    },
    Grouping {
        expression: Box<Expr<'a>>,
    },
    Unary {
        operator: Token<'a>,
        right: Box<Expr<'a>>,
    },
    Variable {
        name: Token<'a>,
    },
}
