use crate::expr_syntax_tree::{Expr};
use crate::token::Token;
use std::fmt;
use crate::ast_printer::AstPrinter;

pub enum Statement<'a> {
    Expression {
        expression: Expr<'a>,
    },
    Print {
        expression: Expr<'a>,
    },
    Var {
        name: Token<'a>,
        initializer: Option<Expr<'a>>,
    }
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
            Statement::Var { name, initializer } => {
                if let Some(init_expr) = initializer {
                    write!(f, "VarStatement(name: {}, initializer: \n\t{}\n)", name.lexeme, ast_printer.visit(init_expr))
                } else {
                    write!(f, "VarStatement(name: {}, initializer: None)", name.lexeme)
                }
            }
        }
    }
}
