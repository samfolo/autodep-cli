extern crate tempfile;

use std::path::PathBuf;
use tempfile::NamedTempFile;

use crate::{node::probe::probe::ModuleSpecifierProbe, test_utils::files::Tempfile};

#[test]
fn test_configure_from_path_valid_config() {
    let tsconfig = Tempfile::create(
        r#"{
        "compilerOptions": {
            "baseUrl": "/absolute/path/to/base"
        }
    }
    "#,
    );

    let probe_result = ModuleSpecifierProbe::new().configure_from_path(&tsconfig.filepath.into());
    assert!(probe_result.is_ok());
}

#[test]
fn test_configure_from_path_invalid_config() {
    let tsconfig = Tempfile::create(r#"invalid json"#);
    let probe_result = ModuleSpecifierProbe::new().configure_from_path(&tsconfig.filepath.into());
    assert!(probe_result.is_err());
}

#[test]
fn test_configure_from_str_valid_config() {
    let tsconfig = r#"
        {
            "compilerOptions": {
                "baseUrl": "/absolute/path/to/base"
            }
        }
        "#;
    let probe_result = ModuleSpecifierProbe::new().configure_from_str(tsconfig);
    assert!(probe_result.is_ok());
}

#[test]
fn test_configure_from_str_invalid_config() {
    let tsconfig = "invalid json";
    let probe_result = ModuleSpecifierProbe::new().configure_from_str(tsconfig);
    assert!(probe_result.is_err());
}
