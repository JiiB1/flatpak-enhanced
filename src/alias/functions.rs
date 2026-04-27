use std::{fs::read_to_string, path::PathBuf};

use crate::{alias::model::Aliases, config::read_or_create_dir, exec::CommandError};

const ALIASES_FILE_EXTENSION: &str = "fpe-aliases";

pub fn target_from_alias(
    config_path: &PathBuf,
    alias: &str,
) -> Result<Option<String>, CommandError> {
    let aliases = list(config_path, &None)?;
    let matching_aliases: Vec<&Aliases> = aliases
        .iter()
        .filter(|Aliases { target: _, aliases }| aliases.iter().any(|a| a == alias))
        .collect();
    match matching_aliases.len() {
        0 => Ok(None),
        1 => Ok(Some(matching_aliases[0].target.clone())),
        _ => Err(CommandError {
            status_code: 11,
            message: format!(
                "Duplicate alias : '{}' is used as an alias for all the following : {}",
                alias,
                aliases
                    .iter()
                    .map(|a| a.target.clone())
                    .collect::<Vec<String>>()
                    .join(",")
            )
            .leak(),
        }),
    }
}

fn list_for_target(config_path: &PathBuf, target: &str) -> Result<Vec<String>, CommandError> {
    read_to_string(
        config_path
            .join("alias")
            .join(format!("{}.{}", target, ALIASES_FILE_EXTENSION)),
    )
    .map(|file_content| {
        file_content
            .split_terminator('\n')
            .map(|line| line.trim().to_string())
            .collect()
    })
    .map_err(|_| CommandError {
        status_code: 10,
        message: format!("No aliases for '{}'", target).leak(),
    })
}

pub fn list(config_path: &PathBuf, target: &Option<String>) -> Result<Vec<Aliases>, CommandError> {
    let mut res = Vec::new();
    match target {
        Some(target) => {
            let target = target_from_alias(config_path, target)?.unwrap_or(target.clone());
            let aliases = list_for_target(config_path, &target)?;
            res.push(Aliases {
                target: target.to_string(),
                aliases,
            });
        }
        None => {
            let files = read_or_create_dir(config_path.join("alias"))?;
            for file in files.iter() {
                if let Some(ext) = file.extension() {
                    if ext == ALIASES_FILE_EXTENSION {
                        if let Some(stem) = file.file_stem() {
                            if let Some(aliases_target) = stem.to_str() {
                                res.push(Aliases {
                                    target: aliases_target.to_string(),
                                    aliases: list_for_target(&config_path, aliases_target)?,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(res)
}
