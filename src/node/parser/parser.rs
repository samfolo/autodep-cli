use swc_common::{sync::Lrc, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, EsConfig, Parser as SwcParser, StringInput, Syntax, TsConfig};

use crate::{common::parser::Parser, errors::ParseError};

pub enum ParseMode {
    TypeScript,
    ECMAScript,
}

pub struct TypeScriptParser {
    source_map: Lrc<SourceMap>,
}

impl TypeScriptParser {
    pub fn new() -> Self {
        Self {
            source_map: Default::default(),
        }
    }
}

impl Parser<Module> for TypeScriptParser {
    fn parse(&self, raw_filepath: &str) -> Result<Module, ParseError> {
        let file_path = std::path::Path::new(raw_filepath);
        let file_map = self.source_map.load_file(&file_path)?;

        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                decorators: true,
                tsx: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*file_map),
            None,
        );

        let mut parser = SwcParser::new_from(lexer);
        parser
            .parse_module()
            .map_err(|e| ParseError::Parse(e.kind().msg().to_string()))
    }
}

pub struct ESParser {
    source_map: Lrc<SourceMap>,
}

impl ESParser {
    pub fn new() -> Self {
        Self {
            source_map: Default::default(),
        }
    }
}

impl Parser<Module> for ESParser {
    fn parse(&self, raw_filepath: &str) -> Result<Module, ParseError> {
        let file_path = std::path::Path::new(raw_filepath);
        let file_map = self.source_map.load_file(&file_path)?;

        let lexer = Lexer::new(
            Syntax::Es(EsConfig {
                decorators: true,
                jsx: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*file_map),
            None,
        );

        let mut parser = SwcParser::new_from(lexer);
        parser
            .parse_module()
            .map_err(|e| ParseError::Parse(e.kind().msg().to_string()))
    }
}
