pub mod cli;
pub mod config;
pub mod node;
pub mod parsers;
pub mod visitors;

use cli::AutodepCli;

fn main() {
    let cli = AutodepCli::new().launch();
    cli.get_matches();
}
