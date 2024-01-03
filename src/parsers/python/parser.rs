use rustpython_parser::{ast, Mode};
use std::fs;

use crate::parsers::{common, errors};

pub struct PythonParser;

impl PythonParser {
    pub fn new() -> Self {
        PythonParser
    }
}

impl common::Parser<ast::Mod> for PythonParser {
    fn parse(&self, file_path: &str) -> Result<ast::Mod, errors::ParseError> {
        let source = fs::read_to_string(file_path).map_err(|e| errors::ParseError::Io(e))?;

        rustpython_parser::parse(&source, Mode::Module, "<embedded>")
            .map_err(|e| errors::ParseError::Parse(e.to_string()))
    }
}
