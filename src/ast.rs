use crate::token::Token;
use crate::token::TokenType;
use crate::token::Literal;

trait ExprNode {
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

pub fn print_example() {
    let expr = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(
                TokenType::Minus,
                "-",
                None,
                1,
            ),
            right: Box::new(Expr::Literal {
                value: Token::new(
                    TokenType::Number,
                    "123",
                    Some(Literal::Number(123.0)),
                    1,
                ),
            }),
        }),
        operator: Token::new(
            TokenType::Star,
            "*",
            None,
            1,
        ),
        right: Box::new(Expr::Grouping { expression: Box::new(Expr::Literal {
            value: Token::new(
                TokenType::Number,
                "45.67",
                Some(Literal::Number(45.67)),
                1,
            ),
        })}),
    };
    println!("{}", expr.visit());
}