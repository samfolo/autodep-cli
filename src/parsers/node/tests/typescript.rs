extern crate tempfile;

use std::io::Write;
use tempfile::NamedTempFile;

use crate::parsers::node::parser::{NodeParser, ParseMode};

// Helper function to create a temporary file with given content
fn create_temp_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temporary file");
    writeln!(file, "{}", content).expect("Failed to write to temporary file");
    file
}

#[test]
fn test_parse_typescript_file() {
    let parser = NodeParser::new();
    let ts_content = r#"let x: number = 10;"#; // Sample TypeScript content
    let temp_file = create_temp_file(ts_content);
    let file_path = temp_file
        .path()
        .to_str()
        .expect("Failed to convert temp file path to string");

    match parser.parse(file_path, ParseMode::TypeScript) {
        Ok(module) => {
            println!("{:#?}", module)
        }
        Err(e) => panic!("Failed to parse TypeScript content: {:?}", e),
    }
}
