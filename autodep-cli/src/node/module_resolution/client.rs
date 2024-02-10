use crate::{cli::handlers::print, errors::ResolverError};
use std::{
    env,
    path::{Path, PathBuf},
};
use swc_common::FileName;
use swc_ecma_loader::{
    resolve::Resolve,
    resolvers::node::{to_absolute_path, NodeModulesResolver},
    resolvers::tsc::TsConfigResolver,
    TargetEnv,
};
use swc_ecma_transforms::modules::path::{
    Config as PathConfig, ImportResolver, NodeImportResolver,
};
use tsconfig::TsConfig;

#[derive(Debug)]
pub struct ModuleResolutionClient {
    base_url: PathBuf,
    pub import_resolver: NodeImportResolver<TsConfigResolver<NodeModulesResolver>>,
}

#[derive(Debug)]
pub enum ModuleType {
    Local(String),
    ThirdParty(String),
    Internal(String),
    CoreOrCustom(String),
}

impl ModuleResolutionClient {
    pub fn new(parsed_config: &TsConfig) -> Self {
        let base_url = parsed_config
            .compiler_options
            .as_ref()
            .and_then(|opts| opts.base_url.clone())
            .map(PathBuf::from)
            .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

        let paths = parsed_config
            .compiler_options
            .as_ref()
            .and_then(|opts| opts.paths.clone())
            .unwrap_or_default();

        let node_modules_resolver =
            NodeModulesResolver::new(TargetEnv::Node, Default::default(), true);

        let tsconfig_resolver = TsConfigResolver::new(
            node_modules_resolver,
            base_url.clone(),
            paths.into_iter().collect(),
        );

        let import_resolver = NodeImportResolver::with_config(
            tsconfig_resolver,
            PathConfig {
                base_dir: Some(base_url.clone()),
                resolve_fully: true,
            },
        );

        Self {
            base_url,
            import_resolver,
        }
    }

    pub fn resolve_import(
        &self,
        filepath: &Path,
        module_specifier: &str,
    ) -> Result<ModuleType, ResolverError> {
        let file_name = FileName::Real(self.base_url.join("tsconfig.json"));

        let resolved_import = self
            .import_resolver
            .resolve_import(&file_name, module_specifier)
            .map_err(|e| ResolverError::ImportResolution(e.to_string()))?;

        // Convert Atom to String for further operations
        let resolved_import_str = resolved_import.to_string();

        if resolved_import_str.starts_with(".") {
            let resolved_path = if resolved_import_str.eq(module_specifier) {
                filepath
                    .parent()
                    .unwrap_or(&Path::new(""))
                    .join(&resolved_import_str)
            } else {
                self.base_url.join(&resolved_import_str)
            };

            let resolved_import_absolute = to_absolute_path(&resolved_path)
                .map_err(|e| ResolverError::PathConversion(e.to_string()))?;

            resolved_import_absolute
                .to_str()
                .ok_or_else(|| ResolverError::NonUtf8Path)
                .map(|s| ModuleType::Local(s.to_string()))
        } else {
            NodeModulesResolver::new(TargetEnv::Node, Default::default(), true)
                .resolve(&FileName::Real(self.base_url.clone()), &resolved_import_str)
                .map_err(|e| ResolverError::ImportResolution(e.to_string()))
                .and_then(|p| match p {
                    FileName::Real(p) => Ok(ModuleType::ThirdParty(
                        p.to_str()
                            .ok_or_else(|| ResolverError::NonUtf8Path)?
                            .to_string(),
                    )),
                    FileName::Internal(i) => Ok(ModuleType::Internal(i)),
                    FileName::Custom(c) => Ok(ModuleType::CoreOrCustom(c)),
                    t => Err(ResolverError::UnexpectedModuleType(t.to_string())),
                })
        }
    }
}
