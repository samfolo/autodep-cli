use clap::{crate_version, Arg, Command};

use crate::{cli::subcommands, flag};

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
            .arg(flag!("verbose", 'v', "Enables verbose output").conflicts_with("silent"))
            .arg(flag!("silent", 's', "Disables all output").conflicts_with("verbose"))
            .arg(flag!("config", 'c', "The config to use with autodep"))
    }
}
