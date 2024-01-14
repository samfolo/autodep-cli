use std::{ffi::OsStr, path::PathBuf};
use tempfile::NamedTempFile;

use crate::{
    node::{parser::ParseMode, probe::probe::ModuleSpecifierProbe},
    test_utils::files::VirtualDirectory,
};

#[test]
fn test_probe_imports_from_file() {
    let root = VirtualDirectory::new(None).unwrap();

    let tsconfig_fixture = root
        .create_file(
            "./",
            "tsconfig",
            Some(OsStr::new("json")),
            Some(&format!(
                r#"{{
                    "compilerOptions": {{
                        "baseUrl": "{}",
                        "paths": {{
                            "@alias/*": ["../aliased/modules/*"]
                        }}
                    }}
                }}"#,
                root.cwd().unwrap().to_str().unwrap()
            )),
        )
        .unwrap();

    let source_file_fixture = root
        .create_file(
            "./",
            "randomFile",
            Some(OsStr::new("ts")),
            Some(
                r#"
                import nodeStdLibImport from 'fs';
                import nodeModuleImport from 'some-node-module';

                import aliasedImport from '@alias/aliasedModule';

                import localFileImport from '../localFile';

                import relativeImport from './relativeModule';


                const add = (a: number, b: number) => a + b;
                
                export const someExport = 42;

                export default add;
                "#,
            ),
        )
        .unwrap();

    let probe = ModuleSpecifierProbe::new().configure_from_path(&tsconfig_fixture.path().into());
    assert!(probe.is_ok());

    let probe = probe.unwrap();
    let imports = probe.probe(
        &source_file_fixture.path().to_str().unwrap(),
        ParseMode::TypeScript,
    );
    assert!(imports.is_ok());

    let imports = imports.unwrap();
    assert_eq!(imports.len(), 5);

    println!("{:#?}", imports);
}
