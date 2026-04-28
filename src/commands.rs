use std::process::Command;

use crate::{
    alias::{alias::AliasCommands, list, model::AliasesCollectionExt},
    config::config_folder_path,
    model::{CmdResult, ResultExt},
};
use clap::Subcommand;

use crate::model::{CmdError, Exec};

#[derive(Subcommand)]
pub enum BaseCommands {
    #[command(external_subcommand)]
    External(Vec<String>),

    #[command(about = "A set of command to manage applications and packages aliases")]
    Alias {
        #[command(subcommand)]
        action: AliasCommands,
    },
}

impl Exec for BaseCommands {
    fn exec(self) -> CmdResult<()> {
        match self {
            BaseCommands::External(args) => {
                // START : alias replacement logic

                let aliases = list(&config_folder_path()?, &None)?;
                let mut args = args;
                for i in 1..args.len() {
                    let arg = &args[i];
                    if !arg.starts_with('-')
                        && let Some(target) = aliases.search_target(arg)?
                    {
                        println!("REPLACED: {} -> {}", arg, target);
                        args[i] = target;
                    }
                }

                // END : alias replacement logic

                let status = Command::new("flatpak")
                    .args(args)
                    .status()
                    .with_cmd_err(1, "Failed to execute flatpak")?;

                let code = status.code().unwrap_or(1);
                if code != 0 {
                    return Err(CmdError::new(
                        code,
                        "Could not execute flatpak : ensure flatpak binaries are accessible via your PATH",
                    ));
                }
                Ok(())
            }
            BaseCommands::Alias { action } => action.exec(),
        }
    }
}
