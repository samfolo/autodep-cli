use std::{env, ffi::OsStr, path::PathBuf};

use crate::errors::ResolverError;

pub fn resolve_tsconfig_path(
    tsconfig_path: Option<&String>,
    target_filepath: &String,
) -> Result<PathBuf, ResolverError> {
    match tsconfig_path {
        Some(path) if path.is_empty() => resolve_nearest_tsconfig(target_filepath),
        Some(path) => {
            let path_buf = PathBuf::from(path);
            if path_buf.is_dir() {
                Ok(path_buf.join("tsconfig.json"))
            } else if path_buf.file_name() == Some(OsStr::new("tsconfig.json")) {
                Ok(path_buf)
            } else {
                Err(ResolverError::InvalidTSConfigPath(path.to_string()))
            }
        }
        None => resolve_nearest_tsconfig(target_filepath),
    }
}

fn resolve_nearest_tsconfig(target_filepath: &String) -> Result<PathBuf, ResolverError> {
    let mut current_dir = env::current_dir()
        .map_err(|_| ResolverError::CurrentDirError)?
        .join(target_filepath);

    while let Some(parent) = current_dir.parent() {
        let tsconfig_path_candidate = parent.join("tsconfig.json");
        if tsconfig_path_candidate.exists() {
            return Ok(tsconfig_path_candidate);
        }
        current_dir = parent.to_path_buf();
    }
    Err(ResolverError::NoTSConfigFound)
}
