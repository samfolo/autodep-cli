use clap::{Arg, ArgGroup, Command};

use crate::cli::args::Argument;

pub fn untangle_subcommand() -> Command {
    Command::new("untangle")
        .about("Rebuild dependency graph for one or more target directories")
        .arg(Arg::from(Argument::Unbounded(
            "dirs",
            None,
            "Path to one or more target directories",
            true,
        )))
        .arg(Arg::from(Argument::Unary(
            "dir",
            Some('d'),
            "Path to a target directory",
            true,
        )))
        .group(
            ArgGroup::new("target-type")
                .args(["dirs", "dir"])
                .required(true),
        )
}
