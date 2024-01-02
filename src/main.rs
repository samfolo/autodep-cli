pub mod cli;
pub mod config;
pub mod parsers;

use cli::AutodepCLI;

fn main() {
    let cli = AutodepCLI::new();
    cli.launch();
}
