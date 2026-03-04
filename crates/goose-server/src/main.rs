mod commands;
mod configuration;
mod error;
mod logging;
mod openapi;
mod routes;
mod state;
mod tunnel;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use goose::agents::validate_extensions;
use goose_mcp::mcp_server_runner::{serve, McpCommand};
#[cfg(feature = "autovisualiser")]
use goose_mcp::AutoVisualiserRouter;
#[cfg(feature = "computercontroller")]
use goose_mcp::ComputerControllerServer;
#[cfg(feature = "memory")]
use goose_mcp::MemoryServer;
#[cfg(feature = "tutorial")]
use goose_mcp::TutorialServer;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the agent server
    Agent,
    /// Run the MCP server
    Mcp {
        #[arg(value_parser = clap::value_parser!(McpCommand))]
        server: McpCommand,
    },
    /// Validate a bundled-extensions JSON file
    #[command(name = "validate-extensions")]
    ValidateExtensions {
        /// Path to the bundled-extensions JSON file
        path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Agent => {
            commands::agent::run().await?;
        }
        Commands::Mcp { server } => {
            logging::setup_logging(Some(&format!("mcp-{}", server.name())))?;
            match server {
                #[cfg(feature = "autovisualiser")]
                McpCommand::AutoVisualiser => serve(AutoVisualiserRouter::new()).await?,
                #[cfg(feature = "computercontroller")]
                McpCommand::ComputerController => serve(ComputerControllerServer::new()).await?,
                #[cfg(feature = "memory")]
                McpCommand::Memory => serve(MemoryServer::new()).await?,
                #[cfg(feature = "tutorial")]
                McpCommand::Tutorial => serve(TutorialServer::new()).await?,
            }
        }
        Commands::ValidateExtensions { path } => {
            match validate_extensions::validate_bundled_extensions(&path) {
                Ok(msg) => println!("{msg}"),
                Err(e) => {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
