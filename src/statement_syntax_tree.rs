use crate::expr_syntax_tree::{Expr};
use std::fmt;
use crate::ast_printer::AstPrinter;

pub enum Statement<'a> {
    Expression {
        expression: Expr<'a>,
    },
    Print {
        expression: Expr<'a>,
    },
}

impl<'a> fmt::Debug for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ast_printer = AstPrinter;

        match self {
            Statement::Expression { expression } => {
                write!(f, "ExpressionStatement(\n\t{}\n)", ast_printer.visit(expression))
            }
            Statement::Print { expression } => {
                write!(f, "PrintStatement(\n\t{}\n)", ast_printer.visit(expression))
            }
        }
    }
}
