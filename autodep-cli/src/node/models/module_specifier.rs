use std::path::PathBuf;

use crate::node::module_resolution::ModuleResolutionClient;

#[derive(Debug)]
pub struct ModuleSpecifier {
    pub(crate) raw: String,
    resolved: Option<String>,
}

impl ModuleSpecifier {
    pub fn from(filepath: &PathBuf, raw: String, client: &ModuleResolutionClient) -> Self {
        let resolved = client
            .resolve_import(filepath, &raw)
            .map(|path| path.to_string())
            .unwrap();

        Self {
            raw,
            resolved: Some(resolved),
        }
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn resolved(&self) -> &Option<String> {
        &self.resolved
    }
}
