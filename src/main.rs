use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check this project for what an LLM thinks is issues
    Check {},
    /// Start the LLM server following the LSP
    Server {},
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Check {} => {
            println!("Running checks...");
        }
        Commands::Server {} => {
            println!("Starting server...");
        }
    }
}
