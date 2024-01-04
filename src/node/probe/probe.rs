use std::path::PathBuf;

use tsconfig::{ConfigError, TsConfig};

use crate::{
    common::parser::Parser,
    node::{
        models::imports::NodeImport,
        module_resolution::ModuleResolutionClient,
        parser::{self, ParseMode, TypeScriptParser},
        visitor::NodeImportsVisitor,
    },
};

struct UninitialisedNodeImportsProbe;

impl UninitialisedNodeImportsProbe {
    pub fn configure_from_path(
        tsconfig_filepath: &PathBuf,
    ) -> Result<NodeImportsProbe, ConfigError> {
        let config = TsConfig::parse_file(tsconfig_filepath)?;
        Ok(NodeImportsProbe {
            config,
            client: ModuleResolutionClient::new(&config),
        })
    }

    pub fn configure_from_str(tsconfig: &str) -> Result<NodeImportsProbe, ConfigError> {
        let config = TsConfig::parse_str(tsconfig)?;
        Ok(NodeImportsProbe {
            config,
            client: ModuleResolutionClient::new(&config),
        })
    }
}

pub struct NodeImportsProbe {
    config: TsConfig,
    client: ModuleResolutionClient,
}

impl NodeImportsProbe {
    pub fn new() -> UninitialisedNodeImportsProbe {
        UninitialisedNodeImportsProbe
    }

    pub fn probe(
        &self,
        raw_filepath: &str,
        mode: ParseMode,
    ) -> Result<Vec<NodeImport>, Box<dyn std::error::Error>> {
        match mode {
            ParseMode::ECMAScript => Ok(vec![]),
            ParseMode::TypeScript => {
                let parser = TypeScriptParser::new();
                let module = parser.parse(raw_filepath)?;

                let visitor = NodeImportsVisitor::new();
                visitor.collect_from(&module);

                let filepath = PathBuf::from(raw_filepath);

                let imports: Vec<_> = visitor
                    .imports()
                    .iter()
                    .map::<String>(|i| self.client.resolve_import(&filepath, i).into())
                    .collect();
                self.client.resolve_import(&filepath, module_specifier)
            }
        }
    }
}
