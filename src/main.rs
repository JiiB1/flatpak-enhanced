use clap::{Parser, Subcommand};
use flatpak_enhanced::alias::AliasCommands;
use std::process::Command;

#[derive(Parser)]
#[command(
    name = "flatpak-enhanced",
    after_help = "Standard Flatpak commands are available and will be passed through.",
    about = "A lightweight wrapper for the flatpak tool",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(external_subcommand)]
    External(Vec<String>),

    #[command(about = "A set of command to manage applications and packages aliases")]
    Alias {
        #[command(subcommand)]
        action: AliasCommands,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::External(args) => {
            let status = Command::new("flatpak")
                .args(&args)
                .status()
                .expect("Failed to execute flatpak");

            std::process::exit(status.code().unwrap_or(1));
        }
        Commands::Alias { action } => match action {
            _ => {}
        },
    }
}
