use crate::expr_syntax_tree::{Expr};
use crate::statement_syntax_tree::{Statement, StatementRef};
use crate::token::Token;

type Output = String;

// Pretty-printer
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) {
        println!("{}", self.visit_with_indent(expr, 0));
    }

    pub fn print_statements(&self, statements: &Vec<StatementRef>) {
        for (i, statement) in statements.iter().enumerate() {
            if i > 0 { println!(""); }
            println!("{}", self.visit_statement_with_indent(statement, 0));
        }
    }

    fn indent(&self, level: usize) -> String {
        "    ".repeat(level)
    }

    fn visit_with_indent(&self, expr: &Expr, level: usize) -> Output {
        match expr {
            Expr::Binary { left, operator, right } => self.visit_binary(left, operator, right, level),
            Expr::Literal { value } => self.visit_literal(value),
            Expr::Grouping { expression } => self.visit_grouping(expression, level),
            Expr::Unary { operator, right } => self.visit_unary(operator, right, level),
            Expr::Variable { name } => self.visit_variable(name),
            Expr::Assign { name, value } => self.visit_assign(name, value, level),
            Expr::LogicOr { left, right } => self.visit_logic_or(left, right, level),
            Expr::LogicAnd { left, right } => self.visit_logic_and(left, right, level),
            Expr::Call { callee, arguments , ..} => self.visit_call(callee, arguments, level),
        }
    }

    fn visit_statement_with_indent(&self, statement: &StatementRef, level: usize) -> Output {
        match statement.as_ref() {
            Statement::Expression { expression } => self.visit_expr_statement(expression, level),
            Statement::Print { expression } => self.visit_print_statement(expression, level),
            Statement::Var { name, initializer } => self.visit_var_statement(name, initializer, level),
            Statement::Block { statements } => self.visit_block_statement(statements, level),
            Statement::If { condition, then_branch, else_branch} => self.visit_if_statement(condition, then_branch, else_branch, level),
            Statement::While { condition, body } => self.visit_while_statement(condition, body, level),
            Statement::Function {name, params, body } => self.visit_function_statement(name, params, body, level),
            Statement::Return { keyword, value } => self.visit_return_statement(keyword, value, level),
        }
    }

    fn visit_binary(&self, left: &Expr, operator: &Token, right: &Expr, level: usize) -> Output {
        let mut s = String::new();
        s.push_str(&format!("({}\n", operator.lexeme));
        s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(left, level + 1)));
        s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(right, level + 1)));
        s.push_str(&format!("{} )", self.indent(level)));
        s
    }

    fn visit_literal(&self, value: &Token) -> Output {
        format!("{}", value.literal.as_ref().unwrap())
    }

    fn visit_grouping(&self, expression: &Expr, level: usize) -> Output {
        let mut s = String::new();
        s.push_str("(group\n");
        s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(expression, level + 1)));
        s.push_str(&format!("{} )", self.indent(level)));
        s
    }

    fn visit_unary(&self, operator: &Token, right: &Expr, level: usize) -> Output {
        let mut s = String::new();
        s.push_str(&format!("({}\n", operator.lexeme));
        s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(right, level + 1)));
        s.push_str(&format!("{} )", self.indent(level)));
        s
    }

    fn visit_variable(&self, name: &Token) -> Output {
        format!("(var {})", name.lexeme)
    }

    fn visit_assign(&self, name: &Token, value: &Expr, level: usize) -> Output {
        let mut s = String::new();
        s.push_str(&format!("(assign {}\n", name.lexeme));
        s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(value, level + 1)));
        s.push_str(&format!("{} )", self.indent(level)));
        s
    }

    fn visit_logic_or(&self, left: &Expr, right: &Expr, level: usize) -> Output {
        let mut s = String::new();
        s.push_str("(or\n");
        s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(left, level + 1)));
        s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(right, level + 1)));
        s.push_str(&format!("{} )", self.indent(level)));
        s
    }

    fn visit_logic_and(&self, left: &Expr, right: &Expr, level: usize) -> Output {
        let mut s = String::new();
        s.push_str("(and\n");
        s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(left, level + 1)));
        s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(right, level + 1)));
        s.push_str(&format!("{} )", self.indent(level)));
        s
    }

    fn visit_call(&self, callee: &Expr, arguments: &Vec<Expr>, level: usize) -> Output {
        let mut s = String::new();
        s.push_str("(call\n");
        s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(callee, level + 1)));
        for argument in arguments {
            s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(argument, level + 1)));
        }
        s.push_str(&format!("{} )", self.indent(level)));
        s
    }

    fn visit_expr_statement(&self, expression: &Expr, level: usize) -> Output {
        let mut s = String::new();
        s.push_str("(expr\n");
        s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(expression, level + 1)));
        s.push_str(&format!("{} )", self.indent(level)));
        s
    }

    fn visit_print_statement(&self, expression: &Expr, level: usize) -> Output {
        let mut s = String::new();
        s.push_str("(print\n");
        s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(expression, level + 1)));
        s.push_str(&format!("{} )", self.indent(level)));
        s
    }

    fn visit_var_statement(&self, name: &Token, initializer: &Option<Expr>, level: usize) -> Output {
        match initializer {
            Some(init_expr) => {
                let mut s = String::new();
                s.push_str(&format!("(var {}\n", name.lexeme));
                s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(init_expr, level + 1)));
                s.push_str(&format!("{} )", self.indent(level)));
                s
            },
            None => format!("(var {} nil)", name.lexeme),
        }
    }

    fn visit_block_statement(&self, statements: &Vec<StatementRef>, level: usize) -> Output {
        let mut s = String::new();
        s.push_str("(block\n");
        for (i, statement) in statements.iter().enumerate() {
            if i > 0 { s.push_str("\n"); }
            s.push_str(&format!("{}{}", self.indent(level + 1), self.visit_statement_with_indent(statement, level + 1)));
        }
        s.push_str(&format!("\n{} )", self.indent(level)));
        s
    }

    fn visit_if_statement(&self, condition: &Expr, then_branch: &StatementRef, else_branch: &Option<StatementRef>, level: usize) -> Output {
        let mut s = String::new();
        s.push_str("(if\n");
        s.push_str(&format!("{}condition: {}\n", self.indent(level + 1), self.visit_with_indent(condition, level + 1)));
        s.push_str(&format!("{}then: {}\n", self.indent(level + 1), self.visit_statement_with_indent(then_branch, level + 1)));
        match else_branch {
            Some(else_stmt) => s.push_str(&format!("{}else: {}\n", self.indent(level + 1), self.visit_statement_with_indent(else_stmt, level + 1))),
            None => s.push_str(&format!("{}else: nil\n", self.indent(level + 1))),
        }
        s.push_str(&format!("{} )", self.indent(level)));
        s
    }

    fn visit_while_statement(&self, condition: &Expr, body: &StatementRef, level: usize) -> Output {
        let mut s = String::new();
        s.push_str("(while\n");
        s.push_str(&format!("{}condition: {}\n", self.indent(level + 1), self.visit_with_indent(condition, level + 1)));
        s.push_str(&format!("{}do: {}\n", self.indent(level + 1), self.visit_statement_with_indent(body, level + 1)));
        s.push_str(&format!("{} )", self.indent(level)));
        s
    }

    fn visit_function_statement(&self, name: &Token, params: &Vec<Token>, body: &Vec<StatementRef>, level: usize) -> Output {
        let mut s = String::new();
        s.push_str(&format!("(function {}\n", name.lexeme));
        s.push_str(&format!("{}params:\n", self.indent(level + 1)));
        for (i, param) in params.iter().enumerate() {
            if i > 0 { s.push_str("\n"); }
            s.push_str(&format!("{}{}", self.indent(level + 2), param.lexeme));
        }
        s.push_str(&format!("\n{}body:\n", self.indent(level + 1)));
        for (i, statement) in body.iter().enumerate() {
            if i > 0 { s.push_str("\n"); }
            s.push_str(&format!("{}{}", self.indent(level + 2), self.visit_statement_with_indent(statement, level + 2)));
        }
        s.push_str(&format!("\n{} )", self.indent(level)));
        s
    }

    fn visit_return_statement(&self, keyword: &Token, value: &Option<Expr>, level: usize) -> Output {
        match value {
            Some(return_expr) => {
                let mut s = String::new();
                s.push_str(&format!("({}\n", keyword.lexeme));
                s.push_str(&format!("{}{}\n", self.indent(level + 1), self.visit_with_indent(return_expr, level + 1)));
                s.push_str(&format!("{} )", self.indent(level)));
                s
            },
            None => format!("(return {} nil)", keyword.lexeme),
        }
    }
}
