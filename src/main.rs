use clap::{Parser, Subcommand};
use std::path::Path;
use std::process;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check this project for what an LLM thinks is issues
    Check {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        extra_args: Vec<String>,
    },
    /// Start the LLM server following the LSP
    Server {},
}

fn check(extra_args: &Vec<String>) {
    for extra_arg in extra_args {
        let path = Path::new(extra_arg);
        if !path.exists() {
            eprintln!("Error: Required path '{extra_arg}' does not exist.");
            process::exit(1);
        }
        if path.is_dir() {
            todo!("Error: Directories not implemented yet '{}'", extra_arg);
        }
        println!("Extra arguments passed: {}", path.display());
    }
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Check { extra_args } => {
            println!("Running checks...");
            check(extra_args);
        }
        Commands::Server {} => {
            println!("Starting server...");
        }
    }
}
