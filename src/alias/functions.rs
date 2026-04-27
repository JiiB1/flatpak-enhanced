use std::{fs::read_to_string, path::PathBuf};

use crate::{
    alias::model::Aliases,
    config::{config_folder_path, read_or_create_dir},
    exec::CommandError,
};

const ALIASES_FILE_EXTENSION: &str = "fpe-aliases";

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
    let files = read_or_create_dir(config_path.join("alias"))?;
    let mut res = Vec::new();
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
    Ok(res)
}
