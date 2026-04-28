use dialoguer::Confirm;
use regex::Regex;
use std::{
    collections::HashSet,
    fs::read_to_string,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};

use crate::{
    alias::model::Aliases,
    config::{get_and_create_file, path_exists, read_and_create_dir, remove_file},
    model::{CmdError, CmdResult, ResultExt},
};

const ALIASES_FILE_EXTENSION: &str = "fpe-aliases";

fn validate_aliases(aliases: Aliases) -> CmdResult<Aliases> {
    for alias in &aliases.aliases {
        if !Regex::new(r"^[\w/-]*$").unwrap().is_match(alias) {
            return Err(CmdError::new(
                17,
                format!(
                    "Incorrect alias '{}' for target '{}'",
                    alias, &aliases.target
                )
                .leak(),
            ));
        }
    }
    Ok(aliases)
}

fn is_target_installed(target: &str) -> CmdResult<bool> {
    Command::new("flatpak")
        .args(["list", "-a", "--columns=application"])
        .stdout(Stdio::piped())
        .spawn()
        .with_cmd_err(12, "Failed to execute flatpak")?
        .stdout
        .ok_or(CmdError::new(
            13,
            "No output for 'flatpak list -a --columns=aplications'",
        ))
        .and_then(|stdout| {
            Command::new("tail")
                .args(["-n", "+2"])
                .stdin(stdout)
                .stdout(Stdio::piped())
                .spawn()
                .with_cmd_err(14, "Failed to execute tail")?
                .stdout
                .ok_or(CmdError::new(14, "No output for 'tail -n +2'"))
                .and_then(|tail_stdout| {
                    Command::new("grep")
                        .arg(target)
                        .stdout(Stdio::null())
                        .stdin(tail_stdout)
                        .status()
                        .with_cmd_err(14, "Failed to execute grep")
                        .and_then(|status| {
                            status
                                .code()
                                .ok_or(CmdError::new(15, "Failed to execute grep"))
                                .and_then(|code| match code {
                                    0 => Ok(true),
                                    1 => Ok(false),
                                    _ => Err(CmdError::new(16, "Error executing grep")),
                                })
                        })
                })
        })
}

fn target_from_alias(config_path: &PathBuf, alias: &str) -> CmdResult<Option<String>> {
    let aliases = list(config_path, &None)?;
    let matching_aliases: Vec<&Aliases> = aliases
        .iter()
        .filter(|Aliases { target: _, aliases }| aliases.iter().any(|a| a == alias))
        .collect();
    match matching_aliases.len() {
        0 => Ok(None),
        1 => Ok(Some(matching_aliases[0].target.clone())),
        _ => Err(CmdError::new(
            11,
            format!(
                "Duplicate alias : '{}' is used as an alias for all the following : {}",
                alias,
                aliases
                    .iter()
                    .map(|a| a.target.clone())
                    .collect::<Vec<String>>()
                    .join(",")
            )
            .leak(),
        )),
    }
}

fn list_for_target(config_path: &PathBuf, target: &str) -> CmdResult<Vec<String>> {
    read_to_string(
        config_path
            .join("alias")
            .join(format!("{}.{}", target, ALIASES_FILE_EXTENSION)),
    )
    .with_cmd_err(10, format!("No aliases for '{}'", target).leak())
    .and_then(|file_content| {
        let Aliases { target: _, aliases } = validate_aliases(Aliases::new(
            target.to_string(),
            file_content
                .split_whitespace()
                .map(|line| line.trim().to_string())
                .collect(),
        ))?;
        Ok(aliases)
    })
}

pub fn create(
    config_path: &PathBuf,
    target: &str,
    aliases: Vec<String>,
    force: bool,
) -> CmdResult<()> {
    if aliases.is_empty() {
        return Ok(());
    }
    let target = target_from_alias(config_path, target)?.unwrap_or(target.to_string());
    let filepath = config_path
        .join("alias")
        .join(format!("{}.{}", target, ALIASES_FILE_EXTENSION));
    if !force
        && !path_exists(&filepath)?
        && !is_target_installed(&target)?
        && !Confirm::new()
            .with_prompt(
                "This target is not an installed application or runtime, continue anyway ?",
            )
            .default(true)
            .interact()
            .with_cmd_err(17, "Error asking to continue")?
    {
        return Ok(());
    }
    let Aliases {
        target,
        mut aliases,
    } = validate_aliases(Aliases::new(target, aliases))?;
    if path_exists(&filepath)? {
        let previous_aliases = list_for_target(config_path, &target)?
            .into_iter()
            .collect::<HashSet<String>>();
        aliases.retain(|alias| !previous_aliases.contains(alias));
    }
    if !aliases.is_empty() {
        get_and_create_file(&filepath, true)?
            .write_all(format!(" {}", aliases.join(" ")).as_bytes())
            .with_cmd_err(
                19,
                format!("Unable to write into file '{}'", filepath.to_string_lossy()).leak(),
            )?;
    }
    Ok(())
}

pub fn remove(config_path: &PathBuf, to_remove: Vec<String>) -> CmdResult<()> {
    let to_keep =
        list(config_path, &None)?
            .into_iter()
            .filter_map(|Aliases { target, aliases }| {
                let len = aliases.len();
                let res: Vec<String> = aliases
                    .into_iter()
                    .filter(|a| !to_remove.contains(a))
                    .collect();
                (res.len() != len).then_some(Aliases::new(target, res))
            });
    for Aliases { target, aliases } in to_keep {
        let filepath = config_path
            .join("alias")
            .join(format!("{}.{}", target, ALIASES_FILE_EXTENSION));
        if aliases.is_empty() {
            remove_file(&filepath)?;
        } else {
            get_and_create_file(&filepath, false)?
                .write_all(aliases.join(" ").as_bytes())
                .with_cmd_err(
                    20,
                    format!("Unable to write into file '{}'", filepath.to_string_lossy()).leak(),
                )?;
        }
    }
    Ok(())
}

pub fn list(config_path: &PathBuf, target: &Option<String>) -> CmdResult<Vec<Aliases>> {
    let mut res = Vec::new();
    if let Some(target) = target {
        let target = target_from_alias(config_path, target)?.unwrap_or(target.clone());
        let aliases = list_for_target(config_path, &target)?;
        res.push(Aliases::new(target.to_string(), aliases));
    } else {
        for file in read_and_create_dir(&config_path.join("alias"))?.iter() {
            if let Some(ext) = file.extension() {
                if ext == ALIASES_FILE_EXTENSION {
                    if let Some(stem) = file.file_stem() {
                        if let Some(aliases_target) = stem.to_str() {
                            res.push(Aliases::new(
                                aliases_target.to_string(),
                                list_for_target(&config_path, aliases_target)?,
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(res)
}
