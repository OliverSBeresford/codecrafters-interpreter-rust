use crate::expr_syntax_tree::{Expr};
use crate::statement_syntax_tree::Statement;
use crate::token::Token;

type Output = String;

// Pretty-printer
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) {
        println!("{}", self.visit(expr));
    }

    pub fn print_statements(&self, statements: &Vec<Statement>) {
        for statement in statements {
            println!("{}", self.visit_statement(statement));
        }
    }

    pub fn visit(&self, expr: &Expr) -> Output {
        match expr {
            Expr::Binary { left, operator, right } => self.visit_binary(left, operator, right),
            Expr::Literal { value } => self.visit_literal(value),
            Expr::Grouping { expression } => self.visit_grouping(expression),
            Expr::Unary { operator, right } => self.visit_unary(operator, right),
            Expr::Variable { name } => self.visit_variable(name),
            Expr::Assign { name, value } => self.visit_assign(name, value)
        }
    }

    fn visit_statement(&self, statement: &Statement) -> Output {
        match statement {
            Statement::Expression { expression } => self.visit_expr_statement(expression),
            Statement::Print { expression } => self.visit_print_statement(expression),
            Statement::Var { name, initializer } => self.visit_var_statement(name, initializer),
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

    fn visit_assign(&self, name: &Token, value: &Expr) -> Output {
        format!("(assign {} {})", name.lexeme, self.visit(value))
    }

    fn visit_expr_statement(&self, expression: &Expr) -> Output {
        format!("(expr {})", self.visit(expression))
    }

    fn visit_print_statement(&self, expression: &Expr) -> Output {
        format!("(print {})", self.visit(expression))
    }

    fn visit_var_statement(&self, name: &Token, initializer: &Option<Expr>) -> Output {
        match initializer {
            Some(init_expr) => format!("(var {} {})", name.lexeme, self.visit(init_expr)),
            None => format!("(var {} nil)", name.lexeme),
        }
    }
}
