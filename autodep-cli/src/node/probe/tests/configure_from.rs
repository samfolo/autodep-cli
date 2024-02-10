use std::{ffi::OsStr, path::PathBuf};

use crate::{
    cli::handlers::print, node::probe::probe::ModuleSpecifierProbe,
    test_utils::files::VirtualDirectory,
};

#[test]
fn test_configure_from_path_valid_config() {
    let mut root = VirtualDirectory::new(None).unwrap();

    let _ = root.create_subdir("test").unwrap();
    let fixture = root.create_file(
        "./test",
        "tsconfig",
        Some(OsStr::new("json")),
        Some(&format!(
            r#"{{
                    "compilerOptions": {{
                        "baseUrl": "{}"
                    }}
                }}"#,
            &root.cwd().unwrap().to_str().unwrap()
        )),
    );
    assert!(fixture.is_ok());
    let fixture = fixture.unwrap();

    let probe_result = ModuleSpecifierProbe::new().configure_from_path(&PathBuf::from(&fixture));
    assert!(probe_result.is_ok());

    root.close();
}

#[test]
fn test_configure_from_path_invalid_config() {
    let mut root = VirtualDirectory::new(None).unwrap();
    let fixture = root.create_file(
        "./",
        "tsconfig",
        Some(OsStr::new("json")),
        Some(r#""invalid json""#),
    );
    assert!(fixture.is_ok());
    let fixture = fixture.unwrap();

    let probe_result = ModuleSpecifierProbe::new().configure_from_path(&PathBuf::from(&fixture));
    assert!(probe_result.is_err());

    root.close();
}

#[test]
fn test_configure_from_str_valid_config() {
    let tsconfig = r#"
        {
            "compilerOptions": {
                "baseUrl": "/absolute/path/to/base"
            }
        }"#;
    let probe_result = ModuleSpecifierProbe::new().configure_from_str(tsconfig);
    assert!(probe_result.is_ok());
}

#[test]
fn test_configure_from_str_invalid_config() {
    let tsconfig = "invalid json";
    let probe_result = ModuleSpecifierProbe::new().configure_from_str(tsconfig);
    assert!(probe_result.is_err());
}
