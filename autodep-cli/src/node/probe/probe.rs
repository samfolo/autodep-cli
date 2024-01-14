use std::{
    env,
    f64::consts::E,
    path::{Path, PathBuf},
    rc::Rc,
};

use clap::builder::Str;
use tsconfig::{CompilerOptions, ConfigError, TsConfig};

use crate::{
    common::parser::Parser,
    node::{
        models::module_specifier::ModuleSpecifier,
        module_resolution::ModuleResolutionClient,
        parser::{ParseMode, TypeScriptParser},
        visitor::ModuleSpecifierVisitor,
    },
};

pub struct UninitialisedModuleSpecifierProbe;

impl UninitialisedModuleSpecifierProbe {
    pub fn configure_from_path(
        &self,
        tsconfig_filepath: &PathBuf,
    ) -> Result<ModuleSpecifierProbe, ConfigError> {
        let mut config = TsConfig::parse_file(tsconfig_filepath)?;

        let current_dir = env::current_dir()?;

        if let Some(compiler_options) = &mut config.compiler_options {
            let tsconfig_dir = tsconfig_filepath.parent().unwrap_or(&current_dir);
            let base_url = compiler_options.base_url.clone().unwrap();
            let absolute_base_url = tsconfig_dir.join(base_url).canonicalize()?;

            compiler_options.base_url = Some(absolute_base_url.to_str().unwrap().to_string());
        }

        let config_rc = Rc::new(config);
        let client = ModuleResolutionClient::new(&config_rc);

        Ok(ModuleSpecifierProbe {
            config: config_rc,
            client,
        })
    }

    pub fn configure_from_str(&self, tsconfig: &str) -> Result<ModuleSpecifierProbe, ConfigError> {
        let config = TsConfig::parse_str(tsconfig)?;
        let config = Rc::new(config);

        Ok(ModuleSpecifierProbe {
            config: Rc::clone(&config),
            client: ModuleResolutionClient::new(&config),
        })
    }
}

#[derive(Debug)]
pub struct ModuleSpecifierProbe {
    config: Rc<TsConfig>,
    client: ModuleResolutionClient,
}

impl ModuleSpecifierProbe {
    pub fn new() -> UninitialisedModuleSpecifierProbe {
        UninitialisedModuleSpecifierProbe
    }

    pub fn probe(
        &self,
        raw_filepath: &str,
        mode: ParseMode,
    ) -> Result<Vec<ModuleSpecifier>, Box<dyn std::error::Error>> {
        match mode {
            ParseMode::ECMAScript => Ok(vec![]),
            ParseMode::TypeScript => {
                let parser = TypeScriptParser::new();
                let module = parser.load_and_parse(raw_filepath)?;

                let filepath = PathBuf::from(raw_filepath);

                let module_specifiers = ModuleSpecifierVisitor::new()
                    .collect_from(&module)
                    .imports()
                    .into_iter()
                    .map(|module_specifier| {
                        ModuleSpecifier::from(&filepath, module_specifier.to_string(), &self.client)
                    })
                    .collect::<Vec<ModuleSpecifier>>();

                Ok(module_specifiers)
            }
        }
    }
}
