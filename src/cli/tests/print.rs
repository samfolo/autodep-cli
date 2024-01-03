#[test]
fn test_no_args() {
    let app = crate::AutodepCli::new().launch();
    let result = app.try_get_matches_from(vec!["autodep", "print"]);

    assert!(
        result.is_err(),
        "Parsing command line arguments should have failed"
    );
}

mod imports {
    #[test]
    fn test_no_args() {
        let app = crate::AutodepCli::new().launch();
        let result = app.try_get_matches_from(vec!["autodep", "print", "imports"]);

        assert!(
            result.is_err(),
            "Parsing command line arguments should have failed"
        );
    }

    mod target {
        #[test]
        fn test_no_args() {
            let app = crate::AutodepCli::new().launch();
            let result = app.try_get_matches_from(vec!["autodep", "print", "imports", "--target"]);

            assert!(
                result.is_err(),
                "Parsing command line arguments should have failed"
            );
        }

        #[test]
        fn test_single() {
            let app = crate::AutodepCli::new().launch();
            let result = app.try_get_matches_from(vec![
                "autodep",
                "print",
                "imports",
                "--target",
                "path/to/file.tsx",
            ]);

            assert!(result.is_ok(), "Failed to parse command line arguments");

            let matches = result.expect("Expected valid matches");
            let print_matches = matches
                .subcommand_matches("print")
                .expect("Expected 'print' subcommand");
            let imports_matches = print_matches
                .subcommand_matches("imports")
                .expect("Expected 'imports' subcommand");
            let target = imports_matches
                .get_one::<String>("target")
                .expect("Expected 'target' argument");

            assert_eq!(target, "path/to/file.tsx");
        }

        #[test]
        fn test_multiple() {
            let app = crate::AutodepCli::new().launch();
            let result = app.try_get_matches_from(vec![
                "autodep",
                "print",
                "imports",
                "--targets",
                "path/to/file.tsx",
                "path/to/other/file.tsx",
            ]);

            assert!(result.is_ok(), "Failed to parse command line arguments");

            let matches = result.expect("Expected valid matches");
            let print_matches = matches
                .subcommand_matches("print")
                .expect("Expected 'print' subcommand");
            let imports_matches = print_matches
                .subcommand_matches("imports")
                .expect("Expected 'imports' subcommand");
            let targets: Vec<_> = imports_matches
                .get_many::<String>("targets")
                .expect("Expected 'targets' argument")
                .collect();

            assert_eq!(targets, vec!["path/to/file.tsx", "path/to/other/file.tsx"]);
        }

        #[test]
        fn test_relative_flag() {
            let app = crate::AutodepCli::new().launch();
            let result = app.try_get_matches_from(vec![
                "autodep",
                "print",
                "imports",
                "--target",
                "path/to/file.tsx",
                "--relative",
            ]);

            assert!(result.is_ok(), "Failed to parse command line arguments");

            let matches = result.expect("Expected valid matches");
            let print_matches = matches
                .subcommand_matches("print")
                .expect("Expected 'print' subcommand");
            let imports_matches = print_matches
                .subcommand_matches("imports")
                .expect("Expected 'imports' subcommand");
            assert!(imports_matches.get_flag("relative"));
        }

        #[test]
        fn test_absolute_flag() {
            let app = crate::AutodepCli::new().launch();
            let result = app.try_get_matches_from(vec![
                "autodep",
                "print",
                "imports",
                "--target",
                "path/to/file.tsx",
                "--absolute",
            ]);

            assert!(result.is_ok(), "Failed to parse command line arguments");

            let matches = result.expect("Expected valid matches");
            let print_matches = matches
                .subcommand_matches("print")
                .expect("Expected 'print' subcommand");
            let imports_matches = print_matches
                .subcommand_matches("imports")
                .expect("Expected 'imports' subcommand");
            assert!(imports_matches.get_flag("absolute"));
        }
    }
}

mod rule {
    #[test]
    fn test_no_args() {
        let app = crate::AutodepCli::new().launch();
        let result = app.try_get_matches_from(vec!["autodep", "print", "rule"]);

        assert!(
            result.is_err(),
            "Parsing command line arguments should have failed"
        );
    }
    mod target {
        #[test]
        fn test_no_args() {
            let app = crate::AutodepCli::new().launch();
            let result = app.try_get_matches_from(vec!["autodep", "print", "rule", "--target"]);

            assert!(
                result.is_err(),
                "Parsing command line arguments should have failed"
            );
        }

        #[test]
        fn test_single() {
            let app = crate::AutodepCli::new().launch();
            let result = app.try_get_matches_from(vec![
                "autodep",
                "print",
                "rule",
                "--target",
                "path/to/file.tsx",
            ]);

            assert!(result.is_ok(), "Failed to parse command line arguments");

            let matches = result.expect("Expected valid matches");
            let print_matches = matches
                .subcommand_matches("print")
                .expect("Expected 'print' subcommand");
            let rule_matches = print_matches
                .subcommand_matches("rule")
                .expect("Expected 'rule' subcommand");
            let target = rule_matches
                .get_one::<String>("target")
                .expect("Expected 'target' argument");

            assert_eq!(target, "path/to/file.tsx");
        }

        #[test]
        fn test_name_only_flag() {
            let app = crate::AutodepCli::new().launch();
            let result = app.try_get_matches_from(vec![
                "autodep",
                "print",
                "rule",
                "--target",
                "path/to/file.tsx",
                "--name-only",
            ]);

            assert!(result.is_ok(), "Failed to parse command line arguments");

            let matches = result.expect("Expected valid matches");
            let print_matches = matches
                .subcommand_matches("print")
                .expect("Expected 'print' subcommand");
            let imports_matches = print_matches
                .subcommand_matches("rule")
                .expect("Expected 'rule' subcommand");
            assert!(imports_matches.get_flag("name-only"));
        }

        #[test]
        fn test_multiple() {
            let app = crate::AutodepCli::new().launch();
            let result = app.try_get_matches_from(vec![
                "autodep",
                "print",
                "rule",
                "--targets",
                "path/to/file.tsx",
                "path/to/other/file.tsx",
            ]);

            assert!(result.is_ok(), "Failed to parse command line arguments");

            let matches = result.expect("Expected valid matches");
            let print_matches = matches
                .subcommand_matches("print")
                .expect("Expected 'print' subcommand");
            let rule_matches = print_matches
                .subcommand_matches("rule")
                .expect("Expected 'rule' subcommand");
            let targets: Vec<_> = rule_matches
                .get_many::<String>("targets")
                .expect("Expected 'targets' argument")
                .collect();

            assert_eq!(targets, vec!["path/to/file.tsx", "path/to/other/file.tsx"]);
        }
    }
}

mod buildfile {
    #[test]
    fn test_no_args() {
        let app = crate::AutodepCli::new().launch();
        let result = app.try_get_matches_from(vec!["autodep", "print", "buildfile"]);

        assert!(
            result.is_err(),
            "Parsing command line arguments should have failed"
        );
    }

    mod target {
        #[test]
        fn test_no_args() {
            let app = crate::AutodepCli::new().launch();
            let result =
                app.try_get_matches_from(vec!["autodep", "print", "buildfile", "--target"]);

            assert!(
                result.is_err(),
                "Parsing command line arguments should have failed"
            );
        }

        #[test]
        fn test_buildfile_target() {
            let app = crate::AutodepCli::new().launch();
            let result = app.try_get_matches_from(vec![
                "autodep",
                "print",
                "buildfile",
                "--target",
                "path/to/BUILD",
            ]);

            assert!(result.is_ok(), "Failed to parse command line arguments");

            let matches = result.expect("Expected valid matches");
            let print_matches = matches
                .subcommand_matches("print")
                .expect("Expected 'print' subcommand");
            let buildfile_matches = print_matches
                .subcommand_matches("buildfile")
                .expect("Expected 'buildfile' subcommand");
            let target = buildfile_matches
                .get_one::<String>("target")
                .expect("Expected 'target' argument");

            assert_eq!(target, "path/to/BUILD");
        }

        #[test]
        fn test_buildfile_multiple_targets() {
            let app = crate::AutodepCli::new().launch();
            let result = app.try_get_matches_from(vec![
                "autodep",
                "print",
                "buildfile",
                "--targets",
                "path/to/BUILD",
                "path/to/other/BUILD.plz",
            ]);

            assert!(result.is_ok(), "Failed to parse command line arguments");

            let matches = result.expect("Expected valid matches");
            let print_matches = matches
                .subcommand_matches("print")
                .expect("Expected 'print' subcommand");
            let buildfile_matches = print_matches
                .subcommand_matches("buildfile")
                .expect("Expected 'buildfile' subcommand");
            let targets: Vec<_> = buildfile_matches
                .get_many::<String>("targets")
                .expect("Expected 'targets' argument")
                .collect();

            assert_eq!(targets, vec!["path/to/BUILD", "path/to/other/BUILD.plz",]);
        }

        #[test]
        fn test_names_only_flag() {
            let app = crate::AutodepCli::new().launch();
            let result = app.try_get_matches_from(vec![
                "autodep",
                "print",
                "buildfile",
                "--targets",
                "path/to/BUILD",
                "path/to/other/BUILD.plz",
                "--names-only",
            ]);

            assert!(result.is_ok(), "Failed to parse command line arguments");

            let matches = result.expect("Expected valid matches");
            let print_matches = matches
                .subcommand_matches("print")
                .expect("Expected 'print' subcommand");
            let buildfile_matches = print_matches
                .subcommand_matches("buildfile")
                .expect("Expected 'buildfile' subcommand");
            assert!(buildfile_matches.get_flag("names-only"));
        }
    }
}
