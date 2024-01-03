use clap::{Arg, ArgGroup, Command};

use crate::{flag, unary, unbounded};

pub fn probe() -> Command {
    Command::new("probe")
        .about("View insights for one or more target build files or directories")
        .arg_required_else_help(true)
        .subcommand(buildfile())
        .subcommand(rule())
}

fn buildfile() -> Command {
    Command::new("buildfile")
        .about("View insights for one or more build files")
        .arg_required_else_help(true)
        .arg(unary!("target", 't', "Path to target build file"))
        .arg(unbounded!(
            "targets",
            "Path to one or more target build files"
        ))
        .group(
            ArgGroup::new("target-plurality")
                .args(["target", "targets"])
                .required(false),
        )
        .arg(flag!("json", "Print as JSON"))
}

fn rule() -> Command {
    Command::new("rule")
        .about("View insights for one or more build rules")
        .arg_required_else_help(true)
        .arg(unary!("target", 't', "Path to target build rule"))
        .arg(unbounded!(
            "targets",
            "Path to one or more target build rules"
        ))
        .group(
            ArgGroup::new("target-plurality")
                .args(["target", "targets"])
                .required(false),
        )
        .arg(flag!("json", "Print as JSON"))
}
