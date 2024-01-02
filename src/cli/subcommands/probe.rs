use clap::{Arg, Command};

use crate::cli::args::Argument;

pub fn probe_subcommand() -> Command {
    Command::new("probe")
        .about("View insights for one or more target build files or directories")
        .subcommand(buildfile_subcommand())
        .subcommand(rule_subcommand())
}

fn buildfile_subcommand() -> Command {
    Command::new("buildfile")
        .about("View insights for a build file")
        .arg_required_else_help(true)
        .arg(Arg::from(Argument::Unary(
            "target",
            Some('t'),
            "Path to target build file",
            true,
        )))
        .arg(Arg::from(Argument::Flag("json", None, "Print as JSON")))
}

fn rule_subcommand() -> Command {
    Command::new("rule")
        .about("View insights for a build rule")
        .arg_required_else_help(true)
        .arg(Arg::from(Argument::Unary(
            "target",
            Some('t'),
            "Path to target build rule",
            true,
        )))
        .arg(Arg::from(Argument::Flag("json", None, "Print as JSON")))
}
