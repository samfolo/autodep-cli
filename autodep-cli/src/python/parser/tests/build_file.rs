use std::io::Write;
use tempfile::NamedTempFile;

use crate::{
    common::parser::Parser,
    python::parser::PythonParser,
    test_utils::files::{ItemConfig, TreeConfig, VirtualDirectory},
};

#[test]
fn test_parse_build_file() {
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

    let source_file_path = tree
        .path()
        .join("BUILD.plz")
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    match parser.load_and_parse(&source_file_path) {
        Ok(build_file) => {
            // println!("{:#?}", build_file) // works! finish test later..
        }
        Err(e) => panic!("Failed to parse TypeScript content: {:?}", e),
    }

    root.close();
}
