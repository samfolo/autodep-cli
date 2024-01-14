use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("Import resolution error: {0}")]
    ImportResolution(String),
    #[error("Canonicalisation error: {0}")]
    Canonicalisation(String),
}
