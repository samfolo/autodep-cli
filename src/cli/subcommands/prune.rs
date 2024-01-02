use clap::{Arg, Command};

use crate::cli::args::Argument;

pub fn prune_subcommand() -> Command {
    Command::new("prune")
        .about("Prune unused build rules in one or more target build files or directories")
        .arg(Arg::from(Argument::Unbounded(
            "targets",
            Some('t'),
            "Path to one or more target build files or directories",
            true,
        )))
}
