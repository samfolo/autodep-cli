extern crate tempfile;

use std::io::Write;
use tempfile::NamedTempFile;

use crate::{common::parser::Parser, node::parser::TypeScriptParser};

// Helper function to create a temporary file with given content
fn create_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(file, "{}", content).expect("Failed to write to temporary file");
    file
}

#[test]
fn test_parse_typescript_file() {
    let parser = TypeScriptParser::new();
    let ts_content = r#"let x: number = 10;"#; // Sample TypeScript content
    let temp_file = create_file(ts_content);
    let file_path = temp_file
        .path()
        .to_str()
        .expect("Failed to convert temp file path to string");

    match parser.load_and_parse(file_path) {
        Ok(module) => {
            println!("{:#?}", module) // works! finish test later..
        }
        Err(e) => panic!("Failed to parse TypeScript content: {:?}", e),
    }
}
