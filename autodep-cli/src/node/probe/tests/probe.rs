use std::ffi::OsStr;

use crate::{
    node::{parser::ParseMode, probe::probe::ModuleSpecifierProbe},
    test_utils::files::{ItemConfig, TreeConfig, VirtualDirectory},
};

#[test]
fn test_probe_imports_from_file() {
    let root = VirtualDirectory::new(None).unwrap();

    let base_dir = root.base_dir().path().to_str().unwrap().to_string();

    let tree = root
        .create_tree(
            &base_dir,
            &TreeConfig {
                items: vec![
                    ItemConfig::File {
                        name: "tsconfig".to_string(),
                        extension: Some("json".to_string()),
                        content: Some(format!(
                            r#"{{
                        "compilerOptions": {{
                            "baseUrl": "{}",
                            "paths": {{
                                "@alias/*": ["../aliased/modules/*"]
                            }}
                        }}
                    }}"#,
                            &base_dir
                        )),
                    },
                    ItemConfig::File {
                        name: "package".to_string(),
                        extension: Some("json".to_string()),
                        content: Some(
                            r#"
                    {
                        "name": "test-package",
                        "version": "0.0.1",
                        "dependencies": {
                            "some-node-module": "1.0.0"
                        }
                    }"#
                            .to_string(),
                        ),
                    },
                    ItemConfig::Directory {
                        name: "node_modules".to_string(),
                        items: vec![ItemConfig::Directory {
                            name: "some-node-module".to_string(),
                            items: vec![
                                ItemConfig::File {
                                    name: "package".to_string(),
                                    extension: Some("json".to_string()),
                                    content: Some(
                                        r#"
                                    {
                                        "name": "some-node-module",
                                        "version": "1.0.0"
                                    }"#
                                        .to_string(),
                                    ),
                                },
                                ItemConfig::File {
                                    name: "index".to_string(),
                                    extension: Some("js".to_string()),
                                    content: Some(
                                        r#"
                                    export default function() {
                                    return 'some-node-module';
                                    }
                                    "#
                                        .to_string(),
                                    ),
                                },
                            ],
                        }],
                    },
                    ItemConfig::Directory {
                        name: "src".to_string(),
                        items: vec![ItemConfig::File {
                            name: "randomFile".to_string(),
                            extension: Some("ts".to_string()),
                            content: Some(
                                r#"
                        import nodeStdLibImport from 'fs';
                        import nodeModuleImport from 'some-node-module';
        
                        import aliasedImport from '@alias/aliasedModule';
        
                        import localFileImport from '../localFile';
        
                        import relativeImport from './relativeModule';
        
        
                        const add = (a: number, b: number) => a + b;
                        
                        export const someExport = 42;
        
                        export default add;
                        "#
                                .to_string(),
                            ),
                        }],
                    },
                ],
            },
        )
        .unwrap();

    let tsconfig_path = VirtualDirectory::derive_path(&tree, vec!["tsconfig.json"]);
    let source_file_path = VirtualDirectory::derive_path(&tree, vec!["src", "randomFile.ts"]);

    let probe = ModuleSpecifierProbe::new().configure_from_path(&tsconfig_path.into());
    assert!(probe.is_ok());

    let probe = probe.unwrap();
    let imports = probe.probe(&source_file_path, ParseMode::TypeScript);
    assert!(imports.is_ok());

    let imports = imports.unwrap();
    assert_eq!(imports.len(), 5);
}
