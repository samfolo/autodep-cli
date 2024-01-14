use clap::{Arg, ArgGroup, Command};

use crate::{unary, unbounded};

pub fn untangle() -> Command {
    Command::new("untangle")
        .about("Rebuild dependency graph for one or more target directories")
        .arg_required_else_help(true)
        .arg(unary!("target", 't', "Path to target directory"))
        .arg(unbounded!(
            "targets",
            "Path to one or more target directories"
        ))
        .group(
            ArgGroup::new("target-plurality")
                .args(["target", "targets"])
                .required(false)
                .multiple(false),
        )
}
