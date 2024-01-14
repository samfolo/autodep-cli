use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("Import resolution error: {0}")]
    ImportResolution(String),
    #[error("Canonicalisation error: {0}")]
    Canonicalisation(String),
    #[error("Path conversion error: {0}")]
    PathConversion(String),
    #[error("No TSConfig found in workspace")]
    NoTSConfigFound,
    #[error("Invalid TSConfig path: {0}")]
    InvalidTSConfigPath(String),
    #[error("Error resolving current dir")]
    CurrentDirError,
    #[error("Non-utf8 path")]
    NonUtf8Path,
}
