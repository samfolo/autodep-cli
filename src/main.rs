pub mod cli;
pub mod common;
pub mod config;
pub mod errors;
pub mod node;
pub mod python;

pub(crate) mod test_utils;

use cli::{handlers::print::imports::handle_print_imports, AutodepCli};

fn main() {
    let cli = AutodepCli::new().launch();
    let arg_matches = cli.get_matches();

    match arg_matches.subcommand() {
        Some(("print", print_subcommands)) => match print_subcommands.subcommand() {
            Some(("imports", imports_subcommands)) => {
                match handle_print_imports(imports_subcommands) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            None => return,
            _ => return,
        },
        Some(("run", _run_subcommands)) => return,
        Some(("probe", _probe_subcommands)) => return,
        Some(("untangle", _untangle_subcommands)) => return,
        Some(("prune", _prune_subcommands)) => return,
        None => return,
        _ => unreachable!(),
    }
}
