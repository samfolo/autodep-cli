use clap::{builder::Str, ArgMatches};
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
            // Should this have a default, or should this fail gracefully?
            let default_tsconfig_path = env::current_dir().unwrap().join("tsconfig.json");

            let probe = ModuleSpecifierProbe::new().configure_from_path(&default_tsconfig_path);
            let probe = probe.unwrap();
            let imports = probe.probe(target_path, ParseMode::TypeScript);
            let imports = imports.unwrap();

            let mut result: Vec<String> = vec![];

            for import in &imports {
                if is_relative {
                    result.push(import.raw().to_owned());
                } else {
                    if let Some(resolved) = import.resolved() {
                        result.push(resolved.to_owned());
                    } else {
                        let unresolvable_import = format!("{} (unresolved)", import.raw());
                        result.push(unresolvable_import);
                    }
                }
            }

            result.sort();

            if is_unique {
                result.dedup();
            }

            for import in result {
                println!("{}", import);
            }
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

    Ok(())
}
