use clap::ArgMatches;

pub fn handle_print_imports(_args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", _args);
    Ok(())
}
