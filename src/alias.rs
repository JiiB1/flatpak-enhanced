use std::{
    collections::HashMap,
    fs::{read_dir, read_to_string},
    path::PathBuf,
};

use crate::model::{Error, Result, ResultExt};

pub type Aliases = HashMap<String, String>;

const FLATPAK_APP_FOLDER: &str = "/var/lib/flatpak/app";
const FLATPAK_APP_METADATA_PATH: &str = "./current/active/metadata";

/// List all application aliases
pub fn list(debug: bool) -> Result<Aliases> {
    let dir_content = read_dir(PathBuf::from(FLATPAK_APP_FOLDER)).with_err(
        10,
        format!("Could not list files in {}", FLATPAK_APP_FOLDER).leak(),
    )?;
    let mut res = HashMap::new();
    for file in dir_content.into_iter() {
        if let Ok(entry) = file {
            let path = entry.path();
            if path.is_dir() {
                if let Some(file_name) = path.file_name() {
                    let application = file_name.to_string_lossy();
                    let metadata_content = read_to_string(path.join(FLATPAK_APP_METADATA_PATH))
                        .with_err(11, format!("Could read metada for {}", application).leak())?;
                    let mut metadata = metadata_content.split_terminator('\n');
                    let alias = metadata.find_map(|line| {
                        if line.starts_with("command=") {
                            return Some(match line.split_once('=') {
                                Some((_, alias)) => Ok(alias),
                                None => Err(Error::new(
                                    12,
                                    format!("Invalid 'command' metadata for {}", application)
                                        .leak(),
                                )),
                            });
                        }
                        None
                    });

                    if let Some(alias) = alias {
                        res.insert(alias?.to_string(), application.to_string());
                    } else if debug {
                        eprintln!(
                            "error: Could not found 'Application -> command' for {}",
                            application
                        )
                    }
                }
            }
        }
    }
    if debug {
        println!(
            "info: found {} application(s) with a suitable alias",
            res.len()
        )
    }
    Ok(res)
}
