use clap::{Arg, ArgGroup, Command};

use crate::{flag, unary, unbounded};

pub fn run() -> Command {
    Command::new("run")
        .about("Update the build file(s) for one or more target modules or directories")
        .arg_required_else_help(true)
        .arg(unary!("target", 't', "Path to target module or directory"))
        .arg(unbounded!(
            "targets",
            "Path to one or more target modules or directories"
        ))
        .group(
            ArgGroup::new("target-plurality")
                .args(["target", "targets"])
                .required(false),
        )
        .arg(flag!("create", "Create a build file if it does not exist"))
        .arg(flag!(
            "update-nearest",
            "Update nearest parent build file for each target file"
        ))
        .group(
            ArgGroup::new("resolution-strategy")
                .args(["create", "update-nearest"])
                .required(false),
        )
        .arg(flag!(
            "canonicalise",
            "Control dependency notation when inserting new dep entries"
        ))
        .arg(flag!("non-recursive", "Skip child directories"))
}
