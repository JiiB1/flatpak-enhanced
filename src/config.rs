use home::home_dir;
use std::{
    env::{self},
    fs::{File, OpenOptions, create_dir_all, exists, read_dir},
    path::PathBuf,
};

use crate::model::{CmdError, CmdResult, ResultExt};

pub fn config_folder_path() -> CmdResult<PathBuf> {
    let env_var_path = env::var("FLATPAK_EHANCED_CONFIG_FOLDER_PATH");
    if env_var_path.is_ok() {
        return Ok(PathBuf::from(env_var_path.unwrap()));
    }
    let home_dir = home_dir();
    if home_dir.is_some() {
        return Ok(home_dir.unwrap().join(".flatpak-enhanced"));
    }
    Err(CmdError::new(
        1,
        "Could not find your home directory. Consider seting the FLATPAK_EHANCED_CONFIG_FOLDER_PATH environement variable.",
    ))
}

pub fn path_exists(path: &PathBuf) -> CmdResult<bool> {
    exists(&path).with_cmd_err(
        2,
        format!("Unable to read path '{}'", path.to_string_lossy()).leak(),
    )
}

pub fn read_and_create_dir(path: &PathBuf) -> CmdResult<Vec<PathBuf>> {
    if !path_exists(path)? {
        return create_dir_all(path)
            .with_cmd_err(
                3,
                format!("Unable to create a directory '{}'", path.to_string_lossy()).leak(),
            )
            .map(|_| Vec::new());
    }
    read_dir(path)
        .with_cmd_err(
            4,
            format!("Unable to list the directory '{}'", path.to_string_lossy()).leak(),
        )
        .map(|content| {
            content
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .collect()
        })
}

pub fn get_and_create_file(path: &PathBuf, append: bool) -> CmdResult<File> {
    OpenOptions::new()
        .write(true)
        .append(append)
        .truncate(!append)
        .create(true)
        .open(path)
        .with_cmd_err(
            5,
            format!("Unable to create file '{}'", path.to_string_lossy()).leak(),
        )
}

pub fn remove_file(path: &PathBuf) -> CmdResult<()> {
    std::fs::remove_file(path).with_cmd_err(
        6,
        format!("Unable to remove file '{}'", path.to_string_lossy()).leak(),
    )
}
