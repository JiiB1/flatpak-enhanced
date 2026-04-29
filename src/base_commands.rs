use std::process::Command;

use crate::{
    alias::{alias::AliasCommands, list, model::AliasesCollectionExt, potential_alias},
    files_management::config_folder_path,
    model::{Result, ResultExt},
};
use clap::Subcommand;

use crate::model::{Error, Exec};

/// All base commands
///
/// # See Also
///
/// - [`Cli`] for usage
#[derive(Subcommand)]
pub enum BaseCommands {
    /// Undefined commands (passed to `flatpak`)
    #[command(external_subcommand)]
    External(Vec<String>),

    /// Access to the aliases management
    #[command(about = "A set of command to manage applications and packages aliases")]
    Alias {
        #[command(subcommand)]
        action: AliasCommands,
    },
}

impl Exec for BaseCommands {
    fn exec(self) -> Result<()> {
        match self {
            BaseCommands::External(mut args) => {
                let has_potential_aliases = args.iter().skip(1).any(|arg| potential_alias(arg));
                if has_potential_aliases {
                    let aliases = list(&config_folder_path()?, &None)?;
                    let alias_index = aliases.build_alias_index();
                    for i in 1..args.len() {
                        let arg = &args[i];
                        if potential_alias(arg) {
                            if let Some(target) = alias_index.get(arg) {
                                args[i] = target.clone();
                            }
                        }
                    }
                }

                let status = Command::new("flatpak")
                    .args(args)
                    .status()
                    .with_err(1, "Failed to execute flatpak")?;

                let code = status.code().unwrap_or(1);
                if code != 0 {
                    return Err(Error::new(
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
