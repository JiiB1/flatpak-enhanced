use clap::Subcommand;
use std::process::Command;

use crate::{
    alias::list,
    model::{Error, Exec, Result, ResultExt},
};

/// All base commands
#[derive(Subcommand)]
pub enum BaseCommands {
    /// Undefined commands (passed to `flatpak`)
    #[command(external_subcommand)]
    External(Vec<String>),
}

impl Exec for BaseCommands {
    fn exec(self, debug: bool) -> Result<()> {
        match self {
            BaseCommands::External(mut args) => {
                if !args.is_empty() {
                    let mut aliases = list(debug)?;
                    if args.len() == 1 {
                        // If a single alias : '$ flatpak run <APP>'
                        if let Some(target) = aliases.remove(&args[0]) {
                            args[0] = target;
                            args.insert(0, "run".to_string());
                        }
                    } else {
                        // Replace all found aliases
                        for i in 1..args.len() {
                            if let Some(target) = aliases.remove(&args[i]) {
                                args[i] = target;
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
        }
    }
}
