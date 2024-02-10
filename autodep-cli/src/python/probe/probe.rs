use crate::{
    common::{parser::Parser, visitor::Visitor},
    python::{parser, rules::BuildRule, visitors::call_exprs},
};

pub struct BuildRuleProbe {}

impl BuildRuleProbe {
    pub fn new() -> Self {
        Self {}
    }

    pub fn probe(raw_filepath: &str) -> Result<Vec<BuildRule>, String> {
        let parser = parser::PythonParser::new();

        let ast = parser
            .load_and_parse(raw_filepath)
            .map_err(|e| e.to_string())?;

        let mut call_exprs_visitor = call_exprs::CallExprsVisitor::new();
        call_exprs_visitor.visit(&ast);
        let call_exprs = call_exprs_visitor.call_exprs();

        let rules = call_exprs
            .iter()
            .map(|call_expr| BuildRule::from(&call_expr))
            .collect::<Vec<BuildRule>>();
        println!("{:#?}", rules);
        Ok(rules)
    }
}
