pub mod cli;
pub mod common;
pub mod config;
pub mod errors;
pub mod node;
pub mod python;

use cli::AutodepCli;

fn main() {
    let cli = AutodepCli::new().launch();
    cli.get_matches();
}
