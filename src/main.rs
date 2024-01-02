pub mod cli;
pub mod config;
pub mod parsers;

use cli::AutodepCli;

fn main() {
    let cli = AutodepCli::new();
    cli.launch();
}
