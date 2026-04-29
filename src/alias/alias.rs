use clap::Subcommand;

use crate::{
    alias::functions::{create, list, remove},
    files_management::config_folder_path,
    model::{Exec, Result},
};

/// All aliases management commands
#[derive(Subcommand)]
pub enum AliasCommands {
    /// To create one or more aliases
    #[command(
        about = "Create alias(es) for an application or a runtime. Existing one will be skipped."
    )]
    Create {
        #[arg(
            long,
            short,
            help = "If specified, it will not be checked if <TARGET> is an installed application or runtime"
        )]
        force: bool,
        #[arg(help = "The application or runtime to create alias(es) for")]
        target: String,
        #[arg(help = "All the aliases to create")]
        aliases: Vec<String>,
    },

    /// To remove one or more aliases
    #[command(about = "Remove alias(es). Existing ones will be skipped.")]
    Remove {
        #[arg(help = "All the aliases to remove")]
        aliases: Vec<String>,
    },

    /// To list all aliases or those of a single target
    #[command(about = "List all aliases")]
    List {
        #[arg(help = "List all the aliases for one specific application or runtime")]
        target: Option<String>,
    },
}

impl Exec for AliasCommands {
    fn exec(self) -> Result<()> {
        let config_path = config_folder_path()?;
        match self {
            AliasCommands::Create {
                target,
                aliases,
                force,
            } => {
                create(&config_path, &target, aliases.into_iter().collect(), force)?;
            }
            AliasCommands::Remove { aliases } => {
                remove(&config_path, aliases)?;
            }
            AliasCommands::List { target } => {
                list(&config_path, &target)?
                    .iter()
                    .for_each(|(target, aliases)| {
                        println!("{}", target);
                        aliases.iter().for_each(|alias| println!("\t{}", alias));
                    });
            }
        };
        Ok(())
    }
}
