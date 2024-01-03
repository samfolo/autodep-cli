use rustpython_parser::{ast, Mode};
use std::collections::HashSet;
use std::path::Path;

use crate::parsers::{common, errors};

pub struct PythonParser {
    filenames: HashSet<String>,
}

impl PythonParser {
    pub fn new(filenames: &[&str]) -> Self {
        let filenames_set = filenames.iter().map(|s| s.to_lowercase()).collect();
        PythonParser {
            filenames: filenames_set,
        }
    }
}

impl common::Parser<ast::Mod> for PythonParser {
    fn parse(&self, raw_file_path: &str) -> Result<ast::Mod, errors::ParseError> {
        let file_path = Path::new(raw_file_path);

        let file_name = file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .ok_or_else(|| {
                errors::ParseError::UnsupportedFile(format!("Invalid file name: {:?}", file_path))
            })?;

        let full_name = file_path
            .file_name()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        if !self.filenames.contains(&file_name) && !self.filenames.contains(&full_name) {
            return Err(errors::ParseError::UnsupportedFile(format!(
                "Unsupported file: {:?}",
                file_path
            )));
        }

        match rustpython_parser::parse(&raw_file_path, Mode::Module, "<embedded>") {
            Ok(program) => Ok(program),
            Err(e) => Err(errors::ParseError::Parse(e.to_string())),
        }
    }
}
