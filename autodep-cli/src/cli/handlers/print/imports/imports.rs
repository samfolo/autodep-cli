use clap::ArgMatches;
use std::{env, path::PathBuf};

use crate::node::{parser::ParseMode, probe::probe::ModuleSpecifierProbe};

pub fn handle_print_imports(args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let target: Option<&String> = args.get_one("target");
    let targets: Option<&Vec<String>> = args.get_one("targets");
    let is_relative = args.get_flag("relative");
    let is_absolute = args.get_flag("absolute");
    let is_unique = args.get_flag("unique");

    match (target, targets) {
        (Some(target_path), None) => {
            // TODO: delete after dev
            let test_tsconfig_path = env::current_dir().unwrap().join("tsconfig.json");

            let probe = ModuleSpecifierProbe::new().configure_from_path(&test_tsconfig_path);
            let probe = probe.unwrap();
            let imports = probe.probe("./test.ts", ParseMode::TypeScript);
            let imports = imports.unwrap();

            for import in imports {
                println!("{:#?}", import);
            }
            // Load TSConfig
            // Load file
            // Parse imports
            // Canonicalise
            // Check if unique flag set
            // Print accordingly
        }
        (None, Some(target_paths)) => {
            for path in target_paths {
                println!("Processing target: {}", path);
            }
        }
        (None, None) => {
            return Err(
                "No target specified. Use --target or --targets to specify target modules.".into(),
            );
        }
        (Some(_), Some(_)) => {
            return Err("Specify either --target or --targets, but not both.".into());
        }
    }

    if is_relative {
        println!("Viewing imports as relative paths");
    }

    if is_absolute {
        println!("Viewing imports as absolute paths");
    }

    if is_unique {
        println!("Viewing imports as a unique list");
    }

    Ok(())
}
