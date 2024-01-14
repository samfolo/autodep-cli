use clap::{
    builder::{ArgPredicate, OsStr},
    Arg, ArgGroup, Command,
};

use crate::{flag, unary, unbounded};

pub fn print() -> Command {
    Command::new("print")
        .about("Print one or more values to the console (stdout)")
        .arg_required_else_help(true)
        .subcommand(print_imports())
        .subcommand(print_rule())
        .subcommand(print_buildfile())
}

fn print_imports() -> Command {
    Command::new("imports")
        .about("View imports for one or more target modules")
        .arg_required_else_help(true)
        .arg(unary!("target", 't', "Path to target module"))
        .arg(unbounded!("targets", "Path to one or more target modules"))
        .group(
            ArgGroup::new("target-plurality")
                .args(["target", "targets"])
                .required(false)
                .multiple(false),
        )
        .arg(flag!("relative", "View imports as relative paths").overrides_with("absolute"))
        .arg(
            flag!("absolute", "View imports as absolute paths")
                .default_value_ifs([
                    ("relative", ArgPredicate::Equals(OsStr::default()), "true"),
                    ("relative", ArgPredicate::IsPresent, "false"),
                ])
                .overrides_with("relative"),
        )
        .group(
            ArgGroup::new("path-format")
                .args(["relative", "absolute"])
                .required(false)
                .multiple(false),
        )
        .arg(flag!(
            "unique",
            "View imports for multiple targets as a unique list"
        ))
}

fn print_rule() -> Command {
    Command::new("rule")
        .about("View the build rule for one or more target modules")
        .arg_required_else_help(true)
        .arg(unary!("target", 't', "Path to target module"))
        .arg(unbounded!("targets", "Path to one or more target modules"))
        .group(
            ArgGroup::new("target-plurality")
                .args(["target", "targets"])
                .required(false)
                .multiple(false),
        )
        .arg(flag!(
            "name-only",
            "Only print the `name` value present in the rule"
        ))
}

fn print_buildfile() -> Command {
    Command::new("buildfile")
        .about("Find nearest build file(s) to one or more target modules")
        .arg_required_else_help(true)
        .arg(unary!("target", 't', "Path to target module"))
        .arg(unbounded!("targets", "Path to one or more target modules"))
        .group(
            ArgGroup::new("target-plurality")
                .args(["target", "targets"])
                .required(false)
                .multiple(false),
        )
        .arg(flag!(
            "names-only",
            "Only print the `name` value(s) present in the build file"
        ))
}
