use crate::expr_syntax_tree::{Expr, ExprVisitor};
use crate::token::Token;

// Expression node trait for printing
pub trait ExprNode {
    fn print(&self) -> String;
}

// Pretty-printer visitor
pub struct AstPrinter;

impl<'a> ExprVisitor<'a> for AstPrinter {
    type Output = String;

    fn visit_binary(&mut self, left: &'a Expr<'a>, operator: &'a Token<'a>, right: &'a Expr<'a>) -> Self::Output {
        format!("({} {} {})", operator.lexeme, left.visit(self), right.visit(self))
    }

    fn visit_literal(&mut self, value: &'a Token<'a>) -> Self::Output {
        format!("{}", value.literal.as_ref().unwrap())
    }

    fn visit_grouping(&mut self, expression: &'a Expr<'a>) -> Self::Output {
        format!("(group {})", expression.visit(self))
    }

    fn visit_unary(&mut self, operator: &'a Token<'a>, right: &'a Expr<'a>) -> Self::Output {
        format!("({} {})", operator.lexeme, right.visit(self))
    }
}

impl<'a> ExprNode for Expr<'a> {
    fn print(&self) -> String {
        let mut printer = AstPrinter;

        // Use the visitor to print this expression
        Expr::visit(self, &mut printer)
    }
}