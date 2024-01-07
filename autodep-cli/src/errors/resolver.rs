use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("Import resolution error: {0}")]
    ImportResolution(String),
}
