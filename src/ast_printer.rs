use crate::expr_syntax_tree::{Expr};
use crate::statement_syntax_tree::{Statement, StatementRef};
use crate::token::Token;

type Output = String;

// Pretty-printer
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) {
        println!("{}", self.visit(expr));
    }

    pub fn print_statements(&self, statements: &Vec<StatementRef>) {
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
            Expr::Assign { name, value } => self.visit_assign(name, value),
            Expr::LogicOr { left, right } => self.visit_logic_or(left, right),
            Expr::LogicAnd { left, right } => self.visit_logic_and(left, right),
            Expr::Call { callee, arguments , ..} => self.visit_call(callee, arguments),
        }
    }

    fn visit_statement(&self, statement: &StatementRef) -> Output {
        match statement.as_ref() {
            Statement::Expression { expression } => self.visit_expr_statement(expression),
            Statement::Print { expression } => self.visit_print_statement(expression),
            Statement::Var { name, initializer } => self.visit_var_statement(name, initializer),
            Statement::Block { statements } => self.visit_block_statement(statements),
            Statement::If { condition, then_branch, else_branch} => self.visit_if_statement(condition, then_branch, else_branch),
            Statement::While { condition, body } => self.visit_while_statement(condition, body),
            Statement::Function {name, params, body } => self.visit_function_statement(name, params, body),
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

    fn visit_logic_or(&self, left: &Expr, right: &Expr) -> Output {
        format!("(or {} {})", self.visit(left), self.visit(right))
    }

    fn visit_logic_and(&self, left: &Expr, right: &Expr) -> Output {
        format!("(and {} {})", self.visit(left), self.visit(right))
    }

    fn visit_call(&self, callee: &Expr, arguments: &Vec<Expr>) -> Output {
        let mut result = format!("(call {}", self.visit(callee));
        for argument in arguments {
            result.push_str(&format!(" {}", self.visit(argument)));
        }
        result.push(')');
        result
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

    fn visit_block_statement(&self, statements: &Vec<StatementRef>) -> Output {
        let mut result = String::from("(block");
        for statement in statements {
            result.push_str(&format!(" {}", self.visit_statement(statement)));
        }
        result.push(')');
        result
    }

    fn visit_if_statement(&self, condition: &Expr, then_branch: &StatementRef, else_branch: &Option<StatementRef>) -> Output {
        match else_branch {
            Some(else_stmt) => format!("(if {} then {} else {})", self.visit(condition), self.visit_statement(then_branch), self.visit_statement(else_stmt)),
            None => format!("(if {} then {} else nil)", self.visit(condition), self.visit_statement(then_branch)),
        }
    }

    fn visit_while_statement(&self, condition: &Expr, body: &StatementRef) -> Output {
        format!("(while {} \ndo {})", self.visit(condition), self.visit_statement(body))
    }

    fn visit_function_statement(&self, name: &Token, params: &Vec<Token>, body: &Vec<StatementRef>) -> Output {
        let mut result = format!("(function {} (params", name.lexeme);
        for param in params {
            result.push_str(&format!(" {}", param.lexeme));
        }
        result.push_str(") (body");
        for statement in body {
            result.push_str(&format!(" {}", self.visit_statement(statement)));
        }
        result.push_str("))");
        result
    }
}
