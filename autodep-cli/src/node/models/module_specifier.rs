use std::path::Path;

use crate::{
    errors::ResolverError,
    node::module_resolution::{ModuleResolutionClient, ModuleType},
};

#[derive(Debug)]
pub struct ModuleSpecifier {
    pub(crate) raw: String,
    resolved: Option<ModuleType>,
}

impl ModuleSpecifier {
    pub fn from(
        filepath: &Path,
        raw: String,
        client: &ModuleResolutionClient,
    ) -> Result<Self, ResolverError> {
        let resolved = client.resolve_import(filepath, &raw)?;

        Ok(Self {
            raw,
            resolved: Some(resolved),
        })
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn resolved(&self) -> &Option<ModuleType> {
        &self.resolved
    }
}
