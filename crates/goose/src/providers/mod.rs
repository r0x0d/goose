pub mod anthropic;
pub mod api_client;
pub mod auto_detect;
pub mod azure;
pub mod azureauth;
pub mod base;
pub mod bedrock;
pub mod canonical;
pub mod catalog;
#[cfg(feature = "cli-providers")]
pub mod chatgpt_codex;
#[cfg(feature = "cli-providers")]
pub mod claude_code;
pub(crate) mod cli_common;
#[cfg(feature = "cli-providers")]
pub mod codex;
#[cfg(feature = "cli-providers")]
pub mod cursor_agent;
pub mod databricks;
pub mod embedding;
pub mod errors;
pub mod formats;
mod gcpauth;
pub mod gcpvertexai;
#[cfg(feature = "cli-providers")]
pub mod gemini_cli;
pub mod githubcopilot;
pub mod google;
mod init;
pub mod lead_worker;
pub mod litellm;
pub mod local_inference;
pub mod oauth;
pub mod ollama;
pub mod openai;
pub mod openai_compatible;
pub mod openrouter;
pub mod provider_registry;
pub mod provider_test;
mod retry;
pub mod sagemaker_tgi;
pub mod snowflake;
pub mod testprovider;
pub mod tetrate;
pub mod toolshim;
pub mod usage_estimator;
pub mod utils;
pub mod venice;
pub mod xai;

pub use init::{
    create, create_with_default_model, create_with_named_model, providers, refresh_custom_providers,
};
pub use retry::{retry_operation, RetryConfig};
