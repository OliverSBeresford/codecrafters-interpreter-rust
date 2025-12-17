use crate::ast::expr::Expr;
use crate::ast::statement::StatementRef;
use crate::lexer::token::Token;

type Output = String;

// Pretty-printer
pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expr) {
        println!("{}", self.visit_with_indent(expr, 0));
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
            Expr::Lambda { params, body } => self.visit_lambda(params, body, level),
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

    fn visit_lambda(&self, params: &Vec<Token>, body: &Vec<StatementRef>, level: usize) -> Output {
        let mut s = String::new();
        s.push_str("(lambda\n");
        s.push_str(&format!("{}(params", self.indent(level + 1)));
        for param in params {
            s.push_str(&format!(" {}", param.lexeme));
        }
        s.push_str(")\n");
        s.push_str(&format!("{}(body\n", self.indent(level + 1)));
        for statement in body {
            s.push_str(&format!("{}{}\n", self.indent(level + 2), format!("{:?}", statement))); // Placeholder for statement printing
        }
        s.push_str(&format!("{} )\n", self.indent(level + 1)));
        s.push_str(&format!("{} )", self.indent(level)));
        s
    }
}
