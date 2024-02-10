use std::ffi::OsStr;

use crate::{
    common::{parser::Parser, visitor::Visitor},
    python::{parser, probe::BuildRuleProbe, rules, visitors::call_exprs},
    test_utils::files::{ItemConfig, TreeConfig, VirtualDirectory},
};

#[test]
fn test_probe_build_rules_from_file() {
    let mut root = VirtualDirectory::new(None).unwrap();

    let base_dir = root.base_dir().path().to_str().unwrap().to_string();

    let tree = root
        .create_tree(
            &base_dir,
            &TreeConfig {
                items: vec![ItemConfig::File {
                    name: "BUILD".to_string(),
                    extension: Some("plz".to_string()),
                    content: Some(
                        r#"
subinclude("//path/to:subinclude")
                
filegroup(
    name = "test",
    srcs = ["test.ts", "test2.ts"],
    visibility = ["//some/dir/..."],
    deps = ["//some/other/dir:dep"],
)
"#
                        .to_string(),
                    ),
                }],
            },
        )
        .unwrap();

    let source_file_path = VirtualDirectory::derive_path(&tree, vec!["BUILD.plz"]);

    let probe = BuildRuleProbe::probe(&source_file_path);
    assert!(probe.is_ok());

    let probe = probe.unwrap();
    assert_eq!(probe.len(), 2);

    assert_eq!(
        probe[0],
        rules::BuildRule::Subinclude {
            subincludes: vec!["//path/to:subinclude".to_string()]
        }
    );

    assert_eq!(
        probe[1],
        rules::BuildRule::Default {
            rule_name: "filegroup".to_string(),
            name: "test".to_string(),
            srcs: vec!["test.ts".to_string(), "test2.ts".to_string()],
            deps: vec!["//some/other/dir:dep".to_string()],
            visibility: vec!["//some/dir/...".to_string()],
            test_only: false,
        }
    );
}
