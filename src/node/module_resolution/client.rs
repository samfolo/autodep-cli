use std::path::PathBuf;
use swc_atoms::Atom;
use swc_common::FileName;
use swc_ecma_loader::{
    resolvers::{node::NodeModulesResolver, tsc::TsConfigResolver},
    TargetEnv,
};
use swc_ecma_transforms::modules::path::{
    Config as PathConfig, ImportResolver, NodeImportResolver,
};

use tsconfig::TsConfig;

use super::errors::ResolverError;

pub struct ModuleResolutionClient {
    pub import_resolver: NodeImportResolver<TsConfigResolver<NodeModulesResolver>>,
}

impl ModuleResolutionClient {
    pub fn new(parsed_config: &TsConfig) -> Self {
        let node_modules_resolver =
            NodeModulesResolver::new(TargetEnv::Node, Default::default(), true);

        let (base_url, paths) = match &parsed_config.compiler_options {
            Some(compiler_options) => (
                compiler_options.base_url.clone().unwrap_or_default(),
                compiler_options.paths.clone().unwrap_or_default(),
            ),
            None => (String::new(), Default::default()),
        };

        let tsconfig_resolver = TsConfigResolver::new(
            node_modules_resolver,
            base_url.clone().into(),
            paths.into_iter().collect(),
        );

        let import_resolver = NodeImportResolver::with_config(
            tsconfig_resolver,
            PathConfig {
                base_dir: Some(PathBuf::from(&base_url)),
                resolve_fully: true,
            },
        );

        Self { import_resolver }
    }

    pub fn resolve_import(
        &self,
        filepath: &PathBuf,
        module_specifier: &str,
    ) -> Result<Atom, ResolverError> {
        self.import_resolver
            .resolve_import(&FileName::Real(filepath.to_path_buf()), module_specifier)
            .map_err(|e| ResolverError::ImportResolution(e.to_string()))
    }
}
