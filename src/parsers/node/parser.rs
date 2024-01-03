use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser, StringInput, Syntax, TsConfig};

use crate::parsers::errors::ParseError;

pub enum ParseMode {
    TypeScript,
    ECMAScript,
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

    pub fn parse(&self, raw_file_path: &str, mode: ParseMode) -> Result<Module, ParseError> {
        let file_path = std::path::Path::new(raw_file_path);
        let file_map = self.source_map.load_file(&file_path)?;

        let lexer = Lexer::new(
            match mode {
                ParseMode::TypeScript => Syntax::Typescript(TsConfig {
                    decorators: true,
                    tsx: true,
                    ..Default::default()
                }),
                ParseMode::ECMAScript => Syntax::Es(EsConfig {
                    decorators: true,
                    jsx: true,
                    ..Default::default()
                }),
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
