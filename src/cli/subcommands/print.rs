use clap::{Arg, ArgGroup, Command};

use crate::cli::{
    args::Argument,
    common::{new_target_arg, new_targets_arg},
};

pub fn print_subcommand() -> Command {
    Command::new("print")
        .about("Print to stdout")
        .subcommand(print_imports_subcommand())
        .subcommand(print_rule_subcommand())
        .subcommand(print_buildfile_subcommand())
}

fn print_imports_subcommand() -> Command {
    Command::new("imports")
        .about("View imports for a target module")
        .arg(new_targets_arg())
        .arg(Arg::from(Argument::Flag(
            "relative",
            None,
            "View imports as relative paths",
        )))
        .arg(Arg::from(Argument::Flag(
            "absolute",
            None,
            "View imports as absolute paths",
        )))
        .arg(Arg::from(Argument::Flag(
            "unique",
            None,
            "View imports for multiple targets as a unique list",
        )))
        .group(
            ArgGroup::new("path-format")
                .args(["relative", "absolute"])
                .required(true),
        )
}

fn print_rule_subcommand() -> Command {
    Command::new("rule")
        .about("View the build rule for a target module")
        .arg(new_target_arg())
        .arg(Arg::from(Argument::Flag(
            "name-only",
            None,
            "Only print the `name` value present in the rule",
        )))
}

fn print_buildfile_subcommand() -> Command {
    Command::new("buildfile")
        .about("Find nearest build file to the target module")
        .arg(new_target_arg())
        .arg(Arg::from(Argument::Flag(
            "names-only",
            None,
            "Only print the `name` value(s) present in the build file",
        )))
}
