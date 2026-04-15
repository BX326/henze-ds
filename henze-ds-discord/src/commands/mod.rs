//! Slash command implementations for the Henze Discord bot.

pub mod new_run;
pub mod place_bet;
pub mod run_status;

pub use new_run::new_run;
pub use place_bet::place_bet;
pub use run_status::run_status;

use serenity::all::{Command, Http};
use tracing::info;

/// Register all slash commands with Discord.
pub async fn register_commands(http: &Http) -> Result<(), serenity::Error> {
    info!("Registering slash commands...");

    let commands = vec![
        new_run::register(),
        place_bet::register(),
        run_status::register(),
    ];

    for command in commands {
        Command::create_global_command(http, command).await?;
    }

    info!("Slash commands registered successfully");
    Ok(())
}
