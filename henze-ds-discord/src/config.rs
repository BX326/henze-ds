//! Configuration constants and system prompt loading.

use std::env;
use std::path::PathBuf;
use tracing::info;

/// Number of bets to include in the daily message.
pub const BETS_PER_MESSAGE: usize = 3;

/// Default database path.
pub const DEFAULT_DATABASE_PATH: &str = "henze_bets.db";

/// Default settlement check interval in seconds (5 minutes).
pub const DEFAULT_SETTLEMENT_INTERVAL_SECS: u64 = 300;

/// Default system prompt (embedded at compile time).
pub const DEFAULT_SYSTEM_PROMPT: &str = include_str!("../system_prompt.txt");

/// Get the database path from environment or use default.
pub fn get_database_path() -> PathBuf {
    env::var("DATABASE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_DATABASE_PATH))
}

/// Get the announcement channel ID (defaults to main DISCORD_CHANNEL_ID if not set).
#[allow(dead_code)]
pub fn get_announcement_channel_id() -> Option<u64> {
    env::var("DISCORD_ANNOUNCEMENT_CHANNEL_ID")
        .or_else(|_| env::var("DISCORD_CHANNEL_ID"))
        .ok()
        .and_then(|s| s.parse().ok())
}

/// Get the settlement check interval in seconds.
pub fn get_settlement_interval_secs() -> u64 {
    env::var("SETTLEMENT_INTERVAL_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_SETTLEMENT_INTERVAL_SECS)
}

/// Load the system prompt with priority: env var > file > default.
pub fn load_system_prompt() -> String {
    // 1. Try environment variable (for secrets/deployment)
    if let Ok(prompt) = env::var("SYSTEM_PROMPT") {
        info!("Using system prompt from SYSTEM_PROMPT env var");
        return prompt;
    }

    // 2. Try custom file (for local development) - check multiple locations
    for path in &[
        "system_prompt.local.txt",
        "henze-ds-discord/system_prompt.local.txt",
    ] {
        if let Ok(custom) = std::fs::read_to_string(path) {
            info!("Using custom system prompt from {}", path);
            return custom;
        }
    }

    // 3. Fall back to default
    DEFAULT_SYSTEM_PROMPT.to_string()
}
