use dialoguer::Confirm;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
    sync::OnceLock,
};

use crate::{
    alias::model::{Aliases, AliasesCollection, AliasesCollectionExt},
    files_management::{get_and_create_file, list_and_create_dir, path_exists, remove_file},
    model::{Error, Result, ResultExt},
};

const ALIASES_FILE_EXTENSION: &str = "fpe-aliases";

static ALIAS_REGEX: OnceLock<Regex> = OnceLock::new();

/// Check if the given string could be an alias
pub fn potential_alias(string: &str) -> bool {
    !string.starts_with('-')
        && ALIAS_REGEX
            .get_or_init(|| Regex::new(r"^[\w/-]+$").unwrap())
            .is_match(string)
}

/// Check if all given aliases are valid. Returns an error if not.
pub fn validate_aliases(aliases: Aliases) -> Result<Aliases> {
    for alias in &aliases.aliases {
        if !potential_alias(alias) {
            return Err(Error::new(
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

/// Try to check if the given target is an installed flatpak's application or runtime.
fn is_target_installed(target: &str) -> Result<bool> {
    let mut flatpak = Command::new("flatpak")
        .args(["list", "-a", "--columns=application"])
        .stdout(Stdio::piped())
        .spawn()
        .with_err(12, "Failed to execute flatpak")?;

    let flatpak_stdout = flatpak
        .stdout
        .take()
        .ok_or(Error::new(13, "No output for 'flatpak list'"))?;

    let mut tail = Command::new("tail")
        .args(["-n", "+2"])
        .stdin(flatpak_stdout)
        .stdout(Stdio::piped())
        .spawn()
        .with_err(14, "Failed to execute tail")?;

    let tail_stdout = tail
        .stdout
        .take()
        .ok_or(Error::new(14, "No output for 'tail -n +2'"))?;

    let grep_status = Command::new("grep")
        .arg(target)
        .stdin(tail_stdout)
        .stdout(Stdio::null())
        .status()
        .with_err(14, "Failed to execute grep")?;

    match grep_status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        Some(_) => Err(Error::new(16, "Error executing grep")),
        None => Err(Error::new(15, "Failed to execute grep")),
    }
}

/// Try to obtain obtain the single target's id linked to the given alias.
///
/// # Returns
///
/// Returns `Ok(Some(String))` if on target has this alias or None.
///
/// # Errors
///
/// Returns `Err(crate::model::Error)` if multiples targets have this same alias
fn target_from_alias(config_path: &PathBuf, alias: &str) -> Result<Option<String>> {
    let matching_aliases = list(config_path, &None)?.targets_from_alias(alias)?;
    match matching_aliases.len() {
        0 => Ok(None),
        1 => Ok(Some(matching_aliases[0].clone())),
        _ => Err(Error::new(
            11,
            format!(
                "Duplicate alias : '{}' is used as an alias for all the following : {}",
                alias,
                matching_aliases.join(",")
            )
            .leak(),
        )),
    }
}

/// List all aliases of a given target
///
/// # Returns
///
/// Returns `Ok(HashSet<String>)` containing all of the given target's aliases
///
/// # Errors
///
/// Returns `Err(crate::model::Error)` if the given doesnt exists
fn list_for_target(config_path: &PathBuf, target: &str) -> Result<HashSet<String>> {
    read_to_string(
        config_path
            .join("alias")
            .join(format!("{}.{}", target, ALIASES_FILE_EXTENSION)),
    )
    .with_err(10, format!("No aliases for '{}'", target).leak())
    .and_then(|file_content| {
        let Aliases { target: _, aliases } = validate_aliases(Aliases {
            target: target.to_string(),
            aliases: file_content
                .split_whitespace()
                .map(|line| line.trim().to_string())
                .collect(),
        })?;
        Ok(aliases)
    })
}

/// Create all given aliases for the given target
pub fn create(
    config_path: &PathBuf,
    target: &str,
    aliases: HashSet<String>,
    force: bool,
) -> Result<()> {
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
            .with_err(17, "Error asking to continue")?
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
            .write_all(
                format!(
                    " {}",
                    aliases.into_iter().collect::<Vec<String>>().join(" ")
                )
                .as_bytes(),
            )
            .with_err(
                19,
                format!("Unable to write into file '{}'", filepath.to_string_lossy()).leak(),
            )?;
    }
    Ok(())
}

/// Remove all given aliases from all targets
pub fn remove(config_path: &PathBuf, to_remove: Vec<String>) -> Result<()> {
    let to_remove_set: HashSet<&String> = to_remove.iter().collect();
    let to_keep: AliasesCollection = list(config_path, &None)?
        .into_iter()
        .filter(|(_, aliases)| {
            aliases.len()
                != aliases
                    .into_iter()
                    .filter(|a| !to_remove.contains(a))
                    .count()
        })
        .collect();
    for (target, aliases) in to_keep {
        let filepath = config_path
            .join("alias")
            .join(format!("{}.{}", target, ALIASES_FILE_EXTENSION));
        let remaining: Vec<_> = aliases
            .into_iter()
            .filter(|a| !to_remove_set.contains(a))
            .collect();
        if remaining.is_empty() {
            remove_file(&filepath)?;
        } else {
            get_and_create_file(&filepath, false)?
                .write_all(remaining.join(" ").as_bytes())
                .with_err(
                    20,
                    format!("Unable to write into file '{}'", filepath.to_string_lossy()).leak(),
                )?;
        }
    }
    Ok(())
}

/// List all aliases by their target of all aliases for the given target's id
pub fn list(config_path: &PathBuf, target: &Option<String>) -> Result<AliasesCollection> {
    let mut res = HashMap::new();
    if let Some(target) = target {
        let target = target_from_alias(config_path, target)?.unwrap_or(target.clone());
        let aliases = list_for_target(config_path, &target)?;
        res.insert(target.to_string(), aliases);
    } else {
        for file in list_and_create_dir(&config_path.join("alias"))?.iter() {
            if let Some(ext) = file.extension() {
                if ext == ALIASES_FILE_EXTENSION {
                    if let Some(stem) = file.file_stem() {
                        if let Some(aliases_target) = stem.to_str() {
                            res.insert(
                                aliases_target.to_string(),
                                list_for_target(&config_path, aliases_target)?,
                            );
                        }
                    }
                }
            }
        }
    }
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_potential_alias() {
        assert!(potential_alias("alias"));
        assert!(potential_alias("alias-test"));
        assert!(potential_alias("alias_test"));
        assert!(potential_alias("SOME_kinda-big_alias"));

        assert!(!potential_alias(""));
        assert!(!potential_alias("\n"));
        assert!(!potential_alias("--not"));
        assert!(!potential_alias("-Not-An-Alias"));
        assert!(!potential_alias("not.an.alias"));
        assert!(!potential_alias("still NOT-an alias"));
    }

    #[test]
    fn test_validate_aliases() {
        let aliases = Aliases::new("target".to_string(), HashSet::new());
        assert!(validate_aliases(aliases).is_ok());
        let aliases = Aliases::new("target".to_string(), HashSet::from(["oK".to_string()]));
        assert!(validate_aliases(aliases).is_ok());
        let aliases = Aliases::new(
            "target".to_string(),
            HashSet::from([
                "valid".to_string(),
                "definitely-ok".to_string(),
                "_strange-but_OKAY".to_string(),
            ]),
        );
        assert!(validate_aliases(aliases).is_ok());

        let aliases = Aliases::new("target".to_string(), HashSet::from(["not.".to_string()]));
        assert!(validate_aliases(aliases).is_err());
        let aliases = Aliases::new(
            "target".to_string(),
            HashSet::from([
                "valid".to_string(),
                "-invalid".to_string(),
                "realy.invalid".to_string(),
            ]),
        );
        assert!(validate_aliases(aliases).is_err());
    }
}
