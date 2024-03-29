extern crate tempfile;

use std::io::Write;
use tempfile::NamedTempFile;

use crate::{
    common::parser::Parser,
    node::parser::TypeScriptParser,
    test_utils::files::{ItemConfig, TreeConfig, VirtualDirectory},
};

#[test]
fn test_parse_typescript_file() {
    let parser = TypeScriptParser::new();

    let mut root = VirtualDirectory::new(None).unwrap();

    let base_dir = root.base_dir().path().to_str().unwrap().to_string();

    let tree = root
        .create_tree(
            &base_dir,
            &TreeConfig {
                items: vec![ItemConfig::File {
                    name: "test".to_string(),
                    extension: Some("ts".to_string()),
                    content: Some(r#"let x: number = 10;"#.to_string()),
                }],
            },
        )
        .unwrap();

    let source_file_path = tree
        .path()
        .join("test.ts")
        .canonicalize()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    match parser.load_and_parse(&source_file_path) {
        Ok(_module) => {
            // println!("{:#?}", module) // works! finish test later..
        }
        Err(e) => panic!("Failed to parse TypeScript content: {:?}", e),
    }

    root.close();
}
