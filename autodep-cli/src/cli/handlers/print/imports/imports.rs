use clap::{builder::Str, ArgMatches};
use std::{env, error::Error, path::PathBuf};

use crate::{
    cli::handlers::common::tsconfig::resolve_tsconfig_path,
    errors::ResolverError,
    node::{module_resolution::ModuleType, parser::ParseMode, probe::probe::ModuleSpecifierProbe},
};

pub fn handle_print_imports(args: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let target: Option<&String> = args.get_one("target");
    let targets: Option<&Vec<String>> = args.get_one("targets");
    let is_relative = args.get_flag("relative");
    let is_unique = args.get_flag("unique");

    match (target, targets) {
        (Some(target_path), None) => {
            let mut result = process_target(target_path, args, is_relative)?;

            if is_unique {
                result.dedup();
            }

            for import in result {
                println!("{}", import);
            }
        }
        (None, Some(target_paths)) => {
            let mut result: Vec<String> = Vec::new();

            for target_path in target_paths {
                let imports = process_target(target_path, args, is_relative)?;
                result.extend(imports);
            }

            if is_unique {
                result.dedup();
            }

            for import in result {
                println!("{}", import);
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

fn process_target(
    target_path: &String,
    args: &ArgMatches,
    is_relative: bool,
) -> Result<Vec<String>, Box<dyn Error>> {
    let tsconfig_path = resolve_tsconfig_path(args.get_one("project"), target_path)?;

    let probe = ModuleSpecifierProbe::new().configure_from_path(&tsconfig_path)?;

    let imports = probe.probe(target_path, ParseMode::TypeScript)?;

    let mut result: Vec<String> = imports
        .iter()
        .map(|import| {
            if is_relative {
                import.raw().to_owned()
            } else if let Some(resolved) = import.resolved() {
                match resolved {
                    ModuleType::Local(specifier)
                    | ModuleType::ThirdParty(specifier)
                    | ModuleType::Internal(specifier)
                    | ModuleType::CoreOrCustom(specifier) => specifier.clone(),
                }
            } else {
                format!("{} (unresolved)", import.raw())
            }
        })
        .collect();

    result.sort();

    Ok(result)
}
