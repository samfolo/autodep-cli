use std::io::Write;
use tempfile::NamedTempFile;

use crate::{
    common::{parser::Parser, visitor::Visitor},
    python::{parser::PythonParser, visitors::call_exprs::call_exprs::CallExprsVisitor},
    test_utils::files::{ItemConfig, TreeConfig, VirtualDirectory},
};

#[test]
fn test_collect_function_calls() {
    let parser = PythonParser::new();

    let mut root = VirtualDirectory::new(None).unwrap();

    let base_dir = root.base_dir().path().to_str().unwrap().to_string();

    let tree = root
        .create_tree(
            &base_dir,
            &TreeConfig {
                items: vec![ItemConfig::File {
                    name: "BUILD".to_string(),
                    extension: Some("plz".to_string()),
                    content: Some(
                        r#"subinclude("//path/to:subinclude")
                
filegroup(
    name = "test",
    srcs = ["test.ts"],
    visibility = ["//some/dir/..."],
    deps = ["//some/other/dir:dep"],
)"#
                        .to_string(),
                    ),
                }],
            },
        )
        .unwrap();

    let source_file_path = VirtualDirectory::derive_path(&tree, vec!["BUILD.plz"]);

    let build_file = parser.load_and_parse(&source_file_path);
    assert!(build_file.is_ok());
    let build_file = build_file.unwrap();

    let mut visitor = CallExprsVisitor::new();
    visitor.visit(&build_file);

    let call_exprs = visitor.call_exprs();
    assert_eq!(call_exprs.len(), 2);

    root.close();
}
