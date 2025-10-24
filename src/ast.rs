use crate::token::Token;

pub trait ExprNode {
    fn visit(&self) -> String;
}

#[derive(Debug)]
pub enum Expr<'a> {
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
    }
}

impl<'a> ExprNode for Expr<'a> {
    fn visit(&self) -> String {
        match self {
            Expr::Binary { left, operator, right } => {
                return format!("({} {} {})", operator.lexeme, left.visit(), right.visit())
            }
            Expr::Literal { value } => {
                // Literal is an Option, so we unwrap it here
                return format!("{}", &value.literal.as_ref().unwrap())
            }
            Expr::Grouping { expression } => {
                return format!("(group {})", expression.visit())
            }
            Expr::Unary { operator, right } => {
                return format!("({} {})", operator.lexeme, right.visit())
            }
        }
    }
}
