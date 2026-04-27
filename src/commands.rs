use std::process::Command;

use crate::alias::alias::AliasCommands;
use clap::Subcommand;

use crate::exec::{CommandError, Exec};

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
    fn exec(&self) -> Result<(), CommandError> {
        match self {
            BaseCommands::External(args) => {
                let status = Command::new("flatpak")
                    .args(args)
                    .status()
                    .expect("Failed to execute flatpak");

                let code = status.code().unwrap_or(1);
                if code != 0 {
                    return Err(CommandError {
                        status_code: code,
                        message: "Could not execute flatpak : ensure flatpak binaries are accessible via your PATH",
                    });
                }
                Ok(())
            }
            BaseCommands::Alias { action } => action.exec(),
        }
    }
}
