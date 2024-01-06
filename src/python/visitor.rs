// use rustpython_parser::ast::{self, Visitor};

// pub struct FunctionCallVisitor {
//     pub function_calls: Vec<ast::Located<ast::Expression>>,
// }

// impl FunctionCallVisitor {
//     pub fn new() -> Self {
//         FunctionCallVisitor {
//             function_calls: Vec::new(),
//         }
//     }
// }

// impl Visitor for FunctionCallVisitor {
//     fn visit_expression(&mut self, expression: &ast::Located<ast::Expression>) {
//         if let ast::Expr::Call { .. } = &expression.node {
//             self.function_calls.push(expression.clone());
//         }
//     }
// }
