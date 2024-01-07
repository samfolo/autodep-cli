#[test]
fn test_help() {
    let mut autodep = crate::cli::AutodepCli::new().launch();
    let help_str = autodep.render_help().to_string();

    assert!(help_str.contains("autodep"));

    assert!(help_str.contains("print"));
    assert!(help_str.contains("run"));
    assert!(help_str.contains("probe"));
    assert!(help_str.contains("untangle"));
    assert!(help_str.contains("prune"));
    assert!(help_str.contains("help"));

    assert!(help_str.contains("--verbose"));
    assert!(help_str.contains("--silent"));
    assert!(help_str.contains("--config"));
    assert!(help_str.contains("--help"));
    assert!(help_str.contains("--version"));
}
