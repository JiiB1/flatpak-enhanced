use clap::Subcommand;

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
