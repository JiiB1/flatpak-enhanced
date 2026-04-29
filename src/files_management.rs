use home::home_dir;
use std::{
    env::{self},
    fs::{File, OpenOptions, create_dir_all, exists, read_dir},
    path::PathBuf,
};

use crate::model::{Error, Result, ResultExt};

/// Try to obtain the path to this process hidden folder containing all its configuration files
///
/// # Returns
///
/// Returns `Ok(PathBuf)` containing the path to the config folder
///
/// # Errors
///
/// Return `Err(crate::model::Error)` if the home dir cannot be found
pub fn config_folder_path() -> Result<PathBuf> {
    env::var("FLATPAK_EHANCED_CONFIG_FOLDER_PATH")
        .and_then(|env_path| Ok(PathBuf::from(env_path)))
        .or_else(|_|
            home_dir()
                .ok_or(Error::new(1,"Could not find your home directory. Consider setting the FLATPAK_EHANCED_CONFIG_FOLDER_PATH environment variable.",))
                .map(|dir| dir.join(".flatpak-enhanced")))
}

/// Try to determine whether a path exits or not
///
/// # Arguments
///
/// * `path` - The path to check
///
/// # Returns
///
/// Returns `Ok(bool)` containing if the file exits or not
///
/// # Errors
///
/// Return `Err(crate::model::Error)` if it can not access this path
pub fn path_exists(path: &PathBuf) -> Result<bool> {
    exists(&path).with_err(
        2,
        format!("Unable to read path '{}'", path.to_string_lossy()).leak(),
    )
}

/// Try to list a directory and create it if it doesnt exists yet
///
/// # Arguments
///
/// * `path` - The path to the directory
///
/// # Returns
///
/// Returns `Ok(bool)` containing all the filepaths in the directory
///
/// # Errors
///
/// Return `Err(crate::model::Error)` if it can not create the directory
pub fn list_and_create_dir(path: &PathBuf) -> Result<Vec<PathBuf>> {
    match read_dir(path) {
        Ok(content) => {
            return Ok(content
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .collect());
        }
        Err(_) => create_dir_all(path)
            .with_err(
                3,
                format!("Unable to create a directory '{}'", path.to_string_lossy()).leak(),
            )
            .map(|_| Vec::new()),
    }
}

/// Try to obtain a file instance and create it if it doesnt exists yet
///
/// # Arguments
///
/// * `path` - The path to the file
/// * `append` - If anything written into this file should be written at the end or replace its content
///
/// # Returns
///
/// Returns `Ok(bool)` containing the file instance
///
/// # Errors
///
/// Return `Err(crate::model::Error)` if it can not create the file
pub fn get_and_create_file(path: &PathBuf, append: bool) -> Result<File> {
    OpenOptions::new()
        .write(true)
        .append(append)
        .truncate(!append)
        .create(true)
        .open(path)
        .with_err(
            5,
            format!("Unable to create file '{}'", path.to_string_lossy()).leak(),
        )
}

/// Try to remove a file
///
/// # Arguments
///
/// * `path` - The path to the file
///
/// # Returns
///
/// Returns `Ok(())` if the file has been removed
///
/// # Errors
///
/// Return `Err(crate::model::Error)` if it can not remove the file
pub fn remove_file(path: &PathBuf) -> Result<()> {
    std::fs::remove_file(path).with_err(
        6,
        format!("Unable to remove file '{}'", path.to_string_lossy()).leak(),
    )
}
