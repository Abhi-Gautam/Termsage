mod prompt;
mod subcommands;

use clap::{Parser, Subcommand};
use dotenv::dotenv;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a terminal command using Gemini API (non-streaming)
    Cmd {
        /// The prompt describing what command you need
        prompt: String,
    },
    /// Get documentation using Gemini API (streaming)
    Doc {
        /// The prompt for documentation help
        prompt: String,
    },
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Cmd { prompt } => {
            if let Err(e) = subcommands::command::generate_command(prompt).await {
                println!("Error generating command: {}", e);
            }
        }
        Commands::Doc { prompt } => {
            if let Err(e) = subcommands::documentation::generate_documentation(prompt).await {
                eprintln!("Error generating documentation: {}", e);
            }
        }
    }
}
