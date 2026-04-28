use clap::Parser;
use flatpak_enhanced::{commands::BaseCommands, model::Exec};

#[derive(Parser)]
#[command(
    name = "flatpak-enhanced",
    after_help = "Standard Flatpak commands are available and will be passed through.",
    about = "A lightweight wrapper for the flatpak tool",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: BaseCommands,
}

fn main() {
    let cli = Cli::parse();
    if let Err(err) = cli.command.exec() {
        eprintln!("error: {}", err.message);
        std::process::exit(err.code);
    }
}
