use rustpython_parser::ast;

use crate::{
    common::{parser::Parser, visitor::Visitor},
    python::{parser, visitors::call_exprs},
};

#[derive(Debug, PartialEq, Clone)]
pub enum BuildRule {
    Subinclude {
        subincludes: Vec<String>,
    },
    Default {
        rule_name: String,
        name: String,
        srcs: Vec<String>,
        deps: Vec<String>,
        visibility: Vec<String>,
        test_only: bool,
    },
}

impl BuildRule {
    pub fn from(call_expr: &ast::ExprCall) -> Self {
        let rule_name = call_expr.func.clone().name_expr().unwrap().id.to_string();

        match rule_name.as_str() {
            "subinclude" => handle_subinclude(call_expr),
            _ => handle_default_build_rule(rule_name, call_expr),
        }
    }
}

enum KeywordArgumentValue {
    Str(String),
    List(Vec<String>),
}

fn resolve_keyword_argument_value(value: &ast::Expr) -> KeywordArgumentValue {
    match &value {
        ast::Expr::Constant(constant) => match &constant.value {
            ast::Constant::Str(s) => KeywordArgumentValue::Str(s.to_string()),
            _ => KeywordArgumentValue::Str("".to_string()), // TODO: handle other types
        },
        ast::Expr::List(list) => KeywordArgumentValue::List(
            list.elts
                .iter()
                .map(|element| match element {
                    ast::Expr::Constant(constant) => match &constant.value {
                        ast::Constant::Str(s) => s.to_string(),
                        _ => "".to_string(), // TODO: handle other types
                    },
                    _ => "".to_string(), // TODO: handle other types
                })
                .collect(),
        ),
        _ => KeywordArgumentValue::List(vec![]), // TODO: handle other types
    }
}

fn handle_subinclude(call_expr: &ast::ExprCall) -> BuildRule {
    let subincludes = call_expr
        .args
        .iter()
        .map(|arg| match arg {
            ast::Expr::Constant(constant) => match &constant.value {
                ast::Constant::Str(s) => s.to_string(),
                _ => "".to_string(), // TODO: handle other types
            },
            _ => "".to_string(), // TODO: handle other types
        })
        .collect();

    BuildRule::Subinclude { subincludes }
}

fn handle_default_build_rule(rule_name: String, call_expr: &ast::ExprCall) -> BuildRule {
    let mut name = String::new();
    let mut srcs = vec![];
    let mut deps = vec![];
    let mut visibility = vec![];
    let mut test_only = false;

    call_expr.keywords.iter().for_each(|keyword| {
        let keyword_name = &keyword.arg.as_ref().unwrap().to_string();
        let keyword_value = match &keyword.value {
            ast::Expr::Constant(constant) => match &constant.value {
                ast::Constant::Str(s) => s.to_string(),
                _ => "".to_string(), // TODO: handle other types
            },
            ast::Expr::List(list) => list
                .elts
                .iter()
                .map(|element| match element {
                    ast::Expr::Constant(constant) => match &constant.value {
                        ast::Constant::Str(s) => s.to_string(),
                        _ => "".to_string(), // TODO: handle other types
                    },
                    _ => "".to_string(), // TODO: handle other types
                })
                .collect(),
            _ => "".to_string(), // TODO: handle other types
        };

        match keyword_name.as_str() {
            "name" => {
                name = match resolve_keyword_argument_value(&keyword.value) {
                    KeywordArgumentValue::Str(s) => s,
                    KeywordArgumentValue::List(list) => list.join(" "),
                }
            }
            "srcs" => {
                srcs = match resolve_keyword_argument_value(&keyword.value) {
                    KeywordArgumentValue::Str(s) => vec![s],
                    KeywordArgumentValue::List(list) => list,
                }
            }
            "deps" => {
                deps = match resolve_keyword_argument_value(&keyword.value) {
                    KeywordArgumentValue::Str(s) => vec![s],
                    KeywordArgumentValue::List(list) => list,
                }
            }
            "visibility" => {
                visibility = match resolve_keyword_argument_value(&keyword.value) {
                    KeywordArgumentValue::Str(s) => vec![s],
                    KeywordArgumentValue::List(list) => list,
                }
            }
            "test_only" => test_only = keyword_value == "True",
            _ => {}
        }
    });

    return BuildRule::Default {
        rule_name,
        name,
        srcs,
        deps,
        visibility,
        test_only,
    };
}
