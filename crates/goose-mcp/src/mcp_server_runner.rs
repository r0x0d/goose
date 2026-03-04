use std::str::FromStr;

use anyhow::Result;
use rmcp::{transport::stdio, ServiceExt};

#[derive(Clone, Debug)]
pub enum McpCommand {
    #[cfg(feature = "autovisualiser")]
    AutoVisualiser,
    #[cfg(feature = "computercontroller")]
    ComputerController,
    #[cfg(feature = "memory")]
    Memory,
    #[cfg(feature = "tutorial")]
    Tutorial,
}

impl FromStr for McpCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().replace(' ', "").as_str() {
            #[cfg(feature = "autovisualiser")]
            "autovisualiser" => Ok(McpCommand::AutoVisualiser),
            #[cfg(feature = "computercontroller")]
            "computercontroller" => Ok(McpCommand::ComputerController),
            #[cfg(feature = "memory")]
            "memory" => Ok(McpCommand::Memory),
            #[cfg(feature = "tutorial")]
            "tutorial" => Ok(McpCommand::Tutorial),
            _ => Err(format!("Invalid command: {}", s)),
        }
    }
}

impl McpCommand {
    pub fn name(&self) -> &str {
        match self {
            #[cfg(feature = "autovisualiser")]
            McpCommand::AutoVisualiser => "autovisualiser",
            #[cfg(feature = "computercontroller")]
            McpCommand::ComputerController => "computercontroller",
            #[cfg(feature = "memory")]
            McpCommand::Memory => "memory",
            #[cfg(feature = "tutorial")]
            McpCommand::Tutorial => "tutorial",
        }
    }
}

pub async fn serve<S>(server: S) -> Result<()>
where
    S: rmcp::ServerHandler,
{
    let service = server.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;

    Ok(())
}
