use clap::Subcommand;

use crate::{
    alias::functions::list,
    config::config_folder_path,
    exec::{CommandError, Exec},
};

#[derive(Subcommand)]
pub enum AliasCommands {
    #[command(
        about = "Create alias(es) for an application or a runtime. Existing one will be skipped."
    )]
    Create {
        #[arg(help = "The application or runtime to create alias(es) for")]
        target: String,
        #[arg(help = "All the aliases to create")]
        aliases: Vec<String>,
    },
    #[command(about = "Remove alias(es). Existing ones will be skipped.")]
    Remove {
        #[arg(help = "All the aliases to remove")]
        aliases: Vec<String>,
    },
    #[command(about = "List all aliases")]
    List {
        #[arg(help = "List all the aliases for one specific application or runtime")]
        target: Option<String>,
    },
}

impl Exec for AliasCommands {
    fn exec(&self) -> Result<(), CommandError> {
        let config_path = config_folder_path()?;
        match self {
            AliasCommands::Create { target, aliases } => {
                // TODO
            }
            AliasCommands::Remove { aliases } => {
                // TODO
            }
            AliasCommands::List { target } => {
                list(&config_path, target).and_then(|all_aliases| {
                    all_aliases.iter().for_each(|aliases| {
                        println!("{}", aliases.target);
                        aliases
                            .aliases
                            .iter()
                            .for_each(|alias| println!("\t{}", alias));
                    });
                    Ok(())
                })?;
            }
        };
        Ok(())
    }
}
