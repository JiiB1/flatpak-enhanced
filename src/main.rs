use clap::Parser;
use flatpak_enhanced::{commands::BaseCommands, exec::Exec};

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
    let res = Cli::parse().command.exec();
    if let Err(err) = res {
        eprintln!("error: {}", err.message);
        std::process::exit(err.status_code);
    }
}
