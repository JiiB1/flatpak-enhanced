use home::home_dir;
use std::{
    env::{self},
    fs::{create_dir_all, exists, read_dir},
    path::PathBuf,
};

use crate::model::{CmdError, CmdResult};

pub fn print_error(message: &str) {
    eprintln!("error: {}", message)
}

pub fn config_folder_path() -> CmdResult<PathBuf> {
    let env_var_path = env::var("FLATPAK_EHANCED_CONFIG_FOLDER_PATH");
    if env_var_path.is_ok() {
        return Ok(PathBuf::from(env_var_path.unwrap()));
    }
    let home_dir = home_dir();
    if home_dir.is_some() {
        return Ok(home_dir.unwrap().join(".flatpak-enhanced"));
    }
    Err(CmdError {
        code: 1,
        message: "Could not find your home directory. Consider seting the FLATPAK_EHANCED_CONFIG_FOLDER_PATH environement variable.",
    })
}

pub fn read_or_create_dir(path: PathBuf) -> CmdResult<Vec<PathBuf>> {
    let path_str = path.to_string_lossy().to_string();
    if !exists(&path).map_err(|_| CmdError {
        code: 2,
        message: format!("Unable to read path '{}'", path_str).leak(),
    })? {
        return create_dir_all(path)
            .map_err(|_| CmdError {
                code: 3,
                message: format!("Unable to create a directory '{}'", path_str).leak(),
            })
            .map(|_| Vec::new());
    }
    read_dir(path)
        .map_err(|_| CmdError {
            code: 4,
            message: format!("Unable to list the directory '{}'", path_str).leak(),
        })
        .map(|content| {
            content
                .filter_map(|entry| entry.ok().map(|e| e.path()))
                .collect()
        })
}
