//! /newrun command - Start a new Henze bet run.

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInteractionResponse, CreateInteractionResponseMessage,
};
use tracing::{error, info};

use crate::db::{self, DbPool};

/// Create the command registration.
pub fn register() -> CreateCommand {
    CreateCommand::new("newrun")
        .description("Start a new Henze bet run with a starting amount")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Number,
                "amount",
                "Starting amount in DKK",
            )
            .required(true)
            .min_number_value(1.0),
        )
}

/// Execute the /newrun command.
pub async fn new_run(ctx: &Context, command: &CommandInteraction, db: &DbPool) -> Result<(), serenity::Error> {
    let guild_id = command
        .guild_id
        .map(|g| g.to_string())
        .unwrap_or_else(|| "dm".to_string());
    let channel_id = command.channel_id.to_string();
    let user_id = command.user.id.to_string();

    // Get the amount parameter
    let amount = command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "amount")
        .and_then(|opt| opt.value.as_f64())
        .unwrap_or(100.0);

    // Create the bet run
    match db::create_bet_run(db, &guild_id, &channel_id, &user_id, amount) {
        Ok(run) => {
            // Get the run number for this guild
            let run_number = db::count_runs_for_guild(db, &guild_id).unwrap_or(1);
            
            info!(
                "Created bet run #{} (id={}) for guild {} by user {} with {} DKK",
                run_number, run.id, guild_id, user_id, amount
            );

            let response = CreateInteractionResponseMessage::new()
                .content(format!(
                    "✅ **Henze Bet Run #{}** created with **{:.2} DKK**!\n\n\
                    Use `/placebet {} <event_id>` to place your first bet.\n\
                    Use `/runstatus {}` to check the run status.",
                    run_number, amount, run_number, run_number
                ))
                .ephemeral(true);

            command
                .create_response(&ctx.http, CreateInteractionResponse::Message(response))
                .await?;
        }
        Err(e) => {
            error!("Failed to create bet run: {:?}", e);
            let response = CreateInteractionResponseMessage::new()
                .content("❌ Failed to create bet run. Please try again.")
                .ephemeral(true);

            command
                .create_response(&ctx.http, CreateInteractionResponse::Message(response))
                .await?;
        }
    }

    Ok(())
}
