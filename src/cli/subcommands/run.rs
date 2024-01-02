use clap::{ArgGroup, Command};

use crate::cli::{args::Argument, common::new_targets_arg};

pub fn run_subcommand() -> Command {
    Command::new("run")
        .about("Update the build file(s) for one or more target modules")
        .arg(new_targets_arg())
        .arg(Argument::Flag(
            "create",
            None,
            "Create a build file if it does not exist",
        ))
        .arg(Argument::Flag(
            "update-nearest",
            None,
            "Update nearest parent build file",
        ))
        .group(
            ArgGroup::new("strategy")
                .args(["create", "update-nearest"])
                .required(false),
        )
        .arg(Argument::Flag(
            "canonicalise",
            None,
            "Control dependency notation when inserting new dependency entries",
        ))
        .arg(Argument::Flag(
            "non-recursive",
            None,
            "Skip child directories",
        ))
}
