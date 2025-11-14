use crate::token::Token;

// Visitor pattern so we can have multiple traversals (printer, interpreter, resolver, ...)
pub trait ExprVisitor<'a> {
    type Output;

    fn visit_binary(&mut self, left: &'a Expr<'a>, operator: &'a Token<'a>, right: &'a Expr<'a>) -> Self::Output;
    fn visit_literal(&mut self, value: &'a Token<'a>) -> Self::Output;
    fn visit_grouping(&mut self, expression: &'a Expr<'a>) -> Self::Output;
    fn visit_unary(&mut self, operator: &'a Token<'a>, right: &'a Expr<'a>) -> Self::Output;
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

impl<'a> Expr<'a> {
    // Double-dispatch entrypoint: run a visitor over this expression and produce its output.
    pub fn visit<T: ExprVisitor<'a>>(&'a self, visitor: &mut T) -> T::Output {
        match self {
            Expr::Binary { left, operator, right } => visitor.visit_binary(left, operator, right),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Unary { operator, right } => visitor.visit_unary(operator, right),
        }
    }
}
