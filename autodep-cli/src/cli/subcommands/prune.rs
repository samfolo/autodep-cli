use clap::{Arg, ArgGroup, Command};

use crate::{unary, unbounded};

pub fn prune() -> Command {
    Command::new("prune")
        .about("Prune unused build rules from one or more target build files or directories")
        .arg_required_else_help(true)
        .arg(unary!(
            "target",
            't',
            "Path to target build file or directory"
        ))
        .arg(unbounded!(
            "targets",
            "Path to one or more target build file or directories"
        ))
        .group(
            ArgGroup::new("target-plurality")
                .args(["target", "targets"])
                .required(false)
                .multiple(false),
        )
}
