use clap::{crate_version, Arg, Command};

use super::{args::Argument, subcommands};

pub struct AutodepCLI;

impl AutodepCLI {
    pub fn new() -> Self {
        AutodepCLI
    }

    pub fn launch(&self) -> clap::ArgMatches {
        Command::new("autodep")
            .version(crate_version!())
            .author("Sam Folorunsho")
            .about("A command line application for managing dependencies in monorepos")
            .arg(
                Arg::new("verbose")
                    .long("verbose")
                    .short('v')
                    .global(true)
                    .num_args(0)
                    .help("Enables verbose output"),
            )
            .arg(
                Arg::new("silent")
                    .long("silent")
                    .short('s')
                    .global(true)
                    .num_args(0)
                    .help("Disables all output"),
            )
            .arg(Arg::from(Argument::Unary(
                "config",
                Some('c'),
                "The config to use with autodep",
                false,
            )))
            .subcommand(subcommands::print_subcommand())
            .subcommand(subcommands::run_subcommand())
            .subcommand(subcommands::probe_subcommand())
            .subcommand(subcommands::untangle_subcommand())
            .subcommand(subcommands::prune_subcommand())
            .get_matches()
    }
}
