use std::path::Path;
use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser, StringInput, Syntax, TsConfig};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parsing error: {0}")]
    Parse(String),
}

pub struct NodeParser {
    source_map: Lrc<SourceMap>,
}

impl NodeParser {
    pub fn new() -> Self {
        NodeParser {
            source_map: Default::default(),
        }
    }

    pub fn parse(&self, file_path: &Path) -> Result<Module, ParseError> {
        let file_map = self.source_map.load_file(&file_path)?;

        let lexer = Lexer::new(
            match file_path.extension().and_then(std::ffi::OsStr::to_str) {
                Some("js") | Some("jsx") | Some("cjs") | Some("mjs") => Syntax::Es(EsConfig {
                    decorators: true,
                    jsx: true,
                    ..Default::default()
                }),
                Some("ts") | Some("tsx") => Syntax::Typescript(TsConfig {
                    decorators: true,
                    tsx: file_path.extension() == Some(std::ffi::OsStr::new("tsx")),
                    ..Default::default()
                }),
                _ => return Err(ParseError::Parse("Unsupported file extension".into())),
            },
            Default::default(),
            StringInput::from(&*file_map),
            None,
        );

        let mut parser = Parser::new_from(lexer);
        parser
            .parse_module()
            .map_err(|e| ParseError::Parse(e.kind().msg().to_string()))
    }
}
