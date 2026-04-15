//! Discord event handler implementation.

use serenity::all::Interaction;
use serenity::prelude::*;
use tracing::{error, info};

use crate::commands::{new_run, place_bet, run_status};
use crate::db::DbPool;

/// Type key for storing database pool in serenity's TypeMap.
pub struct DbPoolKey;

impl TypeMapKey for DbPoolKey {
    type Value = DbPool;
}

/// Discord event handler for bot lifecycle events.
pub struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: serenity::model::gateway::Ready) {
        info!("{} is connected!", ready.user.name);
        
        // Register slash commands
        if let Err(e) = crate::commands::register_commands(&ctx.http).await {
            error!("Failed to register commands: {:?}", e);
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // Get database pool from context
        let db = {
            let data = ctx.data.read().await;
            data.get::<DbPoolKey>().cloned()
        };

        let Some(db) = db else {
            error!("Database pool not found in context");
            return;
        };

        match interaction {
            Interaction::Command(command) => {
                let command_name = command.data.name.as_str();
                info!("Received command: {}", command_name);

                let result = match command_name {
                    "newrun" => new_run::new_run(&ctx, &command, &db).await,
                    "placebet" => place_bet::place_bet(&ctx, &command, &db).await,
                    "runstatus" => run_status::run_status(&ctx, &command, &db).await,
                    _ => {
                        info!("Unknown command: {}", command_name);
                        Ok(())
                    }
                };

                if let Err(e) = result {
                    error!("Error handling command {}: {:?}", command_name, e);
                }
            }
            Interaction::Component(component) => {
                // Handle component interactions (select menus, buttons)
                if component.data.custom_id.starts_with("placebet_select:") {
                    if let Err(e) = place_bet::handle_outcome_selection(&ctx, &component, &db).await {
                        error!("Error handling outcome selection: {:?}", e);
                    }
                }
            }
            _ => {}
        }
    }
}
