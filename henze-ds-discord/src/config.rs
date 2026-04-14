//! Configuration constants and system prompt loading.

use std::env;
use tracing::info;

/// Number of bets to include in the daily message.
pub const BETS_PER_MESSAGE: usize = 3;

/// Default system prompt (embedded at compile time).
pub const DEFAULT_SYSTEM_PROMPT: &str = include_str!("../system_prompt.txt");

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
