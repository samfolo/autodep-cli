extern crate tempfile;

use std::{ffi::OsStr, path::PathBuf};
use tempfile::NamedTempFile;

use crate::{node::probe::probe::ModuleSpecifierProbe, test_utils::files::VirtualDirectory};

#[test]
fn test_configure_from_path_valid_config() {
    let root = VirtualDirectory::new(None).unwrap();

    let fixture = root.create_file(
        ".",
        "tsconfig",
        Some(OsStr::new("json")),
        Some(&format!(
            r#"{{
                    "compilerOptions": {{
                        "baseUrl": "{}"
                    }}
                }}"#,
            root.cwd().unwrap().to_str().unwrap()
        )),
    );
    assert!(fixture.is_ok());
    let fixture = fixture.unwrap();

    let probe_result = ModuleSpecifierProbe::new().configure_from_path(&fixture.path().into());
    assert!(probe_result.is_ok());
}

#[test]
fn test_configure_from_path_invalid_config() {
    let root = VirtualDirectory::new(None).unwrap();
    let fixture = root.create_file(
        "./",
        "tsconfig",
        Some(OsStr::new("json")),
        Some(r#""invalid json""#),
    );
    assert!(fixture.is_ok());
    let fixture = fixture.unwrap();

    let probe_result = ModuleSpecifierProbe::new().configure_from_path(&fixture.path().into());
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
