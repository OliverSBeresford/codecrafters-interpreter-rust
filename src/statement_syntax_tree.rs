use crate::expr_syntax_tree::{Expr};

pub enum Statement<'a> {
    Expression(Expr<'a>),
    Print(Expr<'a>),
}
