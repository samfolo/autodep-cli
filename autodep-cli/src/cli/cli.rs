use clap::{crate_version, Arg, Command};

use crate::{cli::subcommands, flag, unary};

pub struct AutodepCli;

impl AutodepCli {
    pub fn new() -> Self {
        AutodepCli
    }

    pub fn launch(&self) -> clap::Command {
        Command::new("autodep")
            .bin_name("autodep")
            .version(crate_version!())
            .author("Sam Folorunsho")
            .about("A command line application for managing dependencies in monorepos")
            .subcommand(subcommands::print())
            .subcommand(subcommands::run())
            .subcommand(subcommands::probe())
            .subcommand(subcommands::untangle())
            .subcommand(subcommands::prune())
            .arg(
                flag!("verbose", 'v', "Enables verbose output")
                    .conflicts_with("silent")
                    .global(true),
            )
            .arg(
                flag!("silent", 's', "Disables all output")
                    .conflicts_with("verbose")
                    .global(true),
            )
            .arg(
                unary!(
                    "project",
                    'p',
                    "The tsconfig to use when resolving absolute paths. Can be a path to a tsconfig.json file or a directory containing a tsconfig.json file."
                )
                .global(true),
            )
            .arg(unary!("config", 'c', "The config to use with autodep").global(true))
    }
}
