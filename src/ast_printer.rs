use crate::expr_syntax_tree::{Expr};
use crate::token::Token;

type Output = String;

// Pretty-printer
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) {
        println!("{}", self.visit(expr));
    }

    pub fn visit(&self, expr: &Expr) -> Output {
        match expr {
            Expr::Binary { left, operator, right } => self.visit_binary(left, operator, right),
            Expr::Literal { value } => self.visit_literal(value),
            Expr::Grouping { expression } => self.visit_grouping(expression),
            Expr::Unary { operator, right } => self.visit_unary(operator, right),
            Expr::Variable { name } => self.visit_variable(name),
            Expr::Assign { name, value } => {
                format!("(assign {} {})", name.lexeme, self.visit(value))
            }
        }
    }

    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> Output {
        format!("({} {} {})", operator.lexeme, self.visit(left), self.visit(right))
    }

    fn visit_literal(&self, value: &Token) -> Output {
        format!("{}", value.literal.as_ref().unwrap())
    }

    fn visit_grouping(&self, expression: &Expr) -> Output {
        format!("(group {})", self.visit(expression))
    }

    fn visit_unary(&self, operator: &Token, right: &Expr) -> Output {
        format!("({} {})", operator.lexeme, self.visit(right))
    }

    fn visit_variable(&self, name: &Token) -> Output {
        format!("(var {})", name.lexeme)
    }
}
