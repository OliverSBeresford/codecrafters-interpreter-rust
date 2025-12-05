use crate::expr_syntax_tree::{Expr};
use crate::token::Token;
use std::fmt;
use crate::ast_printer::AstPrinter;

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
}

impl fmt::Debug for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ast_printer: AstPrinter = AstPrinter;

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
            Statement::Block { statements } => {
                let mut result = String::from("BlockStatement(\n");
                for statement in statements {
                    result.push_str(&format!("\t{}\n", format!("{:?}", statement).replace("\n", "\n\t")));
                }
                result.push(')');
                write!(f, "{}", result)
            }
            Statement::If { condition, then_branch, else_branch } => {
                if let Some(else_stmt) = else_branch {
                    write!(f, "IfStatement(\n\tcondition: {},\n\tthen_branch: {:?},\n\telse_branch: {:?}\n)", 
                        ast_printer.visit(condition), then_branch, else_stmt)
                } else {
                    write!(f, "IfStatement(\n\tcondition: {},\n\tthen_branch: {:?},\n\telse_branch: None\n)", 
                        ast_printer.visit(condition), then_branch)
                }
            }
            Statement::While { condition, body } => {
                write!(f, "WhileStatement(\n\tcondition: {},\n\tbody: {:?}\n)", 
                    ast_printer.visit(condition), body)
            }
        }
    }
}
