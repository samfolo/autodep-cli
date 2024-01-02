use clap::Arg;

use crate::cli::args::Argument;

pub fn new_target_arg() -> Arg {
    Arg::from(Argument::Unary(
        "target",
        Some('t'),
        "Path to target module",
        true,
    ))
}

pub fn new_targets_arg() -> Arg {
    Arg::from(Argument::Unbounded(
        "targets",
        Some('t'),
        "Path to one or more target modules",
        true,
    ))
}
