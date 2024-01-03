use std::{collections::HashSet, path::Path};
use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser, StringInput, Syntax, TsConfig};

use crate::parsers::{common, errors};

pub struct NodeParser {
    source_map: Lrc<SourceMap>,
    es_extensions: HashSet<String>,
    ts_extensions: HashSet<String>,
}

impl NodeParser {
    pub fn new(es_extensions: &[&str], ts_extensions: &[&str]) -> Self {
        NodeParser {
            source_map: Default::default(),
            es_extensions: es_extensions.iter().map(|&ext| ext.to_string()).collect(),
            ts_extensions: ts_extensions.iter().map(|&ext| ext.to_string()).collect(),
        }
    }
}

impl common::Parser<Module> for NodeParser {
    fn parse(&self, raw_file_path: &str) -> Result<Module, errors::ParseError> {
        let file_path = Path::new(raw_file_path);
        let file_map = self.source_map.load_file(&file_path)?;
        let extension = file_path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .map(|s| s.to_lowercase())
            .unwrap_or_default();

        let lexer = Lexer::new(
            if self.es_extensions.contains(&extension) {
                Syntax::Es(EsConfig {
                    decorators: true,
                    jsx: true,
                    ..Default::default()
                })
            } else if self.ts_extensions.contains(&extension) {
                Syntax::Typescript(TsConfig {
                    decorators: true,
                    tsx: extension == "tsx",
                    ..Default::default()
                })
            } else {
                return Err(errors::ParseError::Parse(
                    "Unsupported file extension".into(),
                ));
            },
            Default::default(),
            StringInput::from(&*file_map),
            None,
        );

        let mut parser = Parser::new_from(lexer);
        parser
            .parse_module()
            .map_err(|e| errors::ParseError::Parse(e.kind().msg().to_string()))
    }
}
