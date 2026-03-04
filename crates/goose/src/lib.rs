pub mod action_required_manager;
pub mod agents;
pub mod builtin_extension;
pub mod config;
pub mod context_mgmt;
pub mod conversation;
pub mod dictation;
pub mod download_manager;
pub mod execution;
pub mod gateway;
pub mod goose_apps;
pub mod hints;
pub mod logging;
pub mod mcp_utils;
pub mod model;
pub mod oauth;
#[cfg(feature = "otel")]
pub mod otel;
pub mod permission;
#[cfg(feature = "telemetry")]
pub mod posthog;
#[cfg(not(feature = "telemetry"))]
pub mod posthog {
    pub const TELEMETRY_ENABLED_KEY: &str = "GOOSE_TELEMETRY_ENABLED";
    pub fn get_telemetry_choice() -> Option<bool> {
        Some(false)
    }
    pub fn is_telemetry_enabled() -> bool {
        false
    }
    pub fn set_session_context(_interface: &str, _is_resumed: bool) {}
    pub fn emit_session_started() {}
    pub fn emit_error(_error_type: &str, _error_message: &str) {}
    pub fn emit_custom_slash_command_used() {}
    pub fn classify_error(_error: &str) -> &'static str {
        "unknown"
    }
    pub async fn emit_event(
        _event_name: &str,
        _properties: std::collections::HashMap<String, serde_json::Value>,
    ) -> Result<(), String> {
        Ok(())
    }
}
pub mod prompt_template;
pub mod providers;
pub mod recipe;
pub mod recipe_deeplink;
pub mod scheduler;
pub mod scheduler_trait;
pub mod security;
pub mod session;
pub mod session_context;
pub mod slash_commands;
pub mod subprocess;
pub mod token_counter;
pub mod tool_inspection;
pub mod tool_monitor;
pub mod tracing;
pub mod utils;
