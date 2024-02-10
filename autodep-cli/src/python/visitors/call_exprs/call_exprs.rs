use rustpython_parser::ast;

use crate::common::visitor::Visitor;

pub struct CallExprsVisitor {
    call_exprs: Vec<ast::ExprCall>,
}

impl Visitor<ast::Mod> for CallExprsVisitor {
    fn visit(&mut self, node: &ast::Mod) {
        self.visit_module(&node);
    }
}

impl CallExprsVisitor {
    pub fn new() -> Self {
        CallExprsVisitor {
            call_exprs: Vec::new(),
        }
    }

    pub fn call_exprs(&self) -> &Vec<ast::ExprCall> {
        &self.call_exprs
    }
}

impl CallExprsVisitor {
    fn visit_module(&mut self, node: &ast::Mod) {
        if let Some(module) = node.as_module() {
            module.body.iter().for_each(|stmt| self.visit_stmt(stmt));
        }
    }

    fn visit_stmt(&mut self, node: &ast::Stmt) {
        match node {
            ast::Stmt::Expr(expr) => self.visit_expr(expr),
            _ => return,
        }
    }

    fn visit_expr(&mut self, node: &ast::StmtExpr) {
        match node.value.as_ref() {
            ast::Expr::Call(call) => self.visit_expr_call(call),
            _ => return,
        }
    }

    fn visit_expr_call(&mut self, node: &ast::ExprCall) {
        self.call_exprs.push(node.clone());
    }
}
