//! /runstatus command - Show the status of a Henze bet run.

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use tracing::error;

use crate::db::{self, BetStatus, DbPool, RunStatus};

/// Create the command registration.
pub fn register() -> CreateCommand {
    CreateCommand::new("runstatus")
        .description("Show the status of a Henze bet run")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "run",
                "Run number (optional - shows latest active run if omitted)",
            )
            .required(false)
            .min_int_value(1),
        )
}

/// Execute the /runstatus command.
pub async fn run_status(ctx: &Context, command: &CommandInteraction, db: &DbPool) -> Result<(), serenity::Error> {
    let guild_id = command
        .guild_id
        .map(|g| g.to_string())
        .unwrap_or_else(|| "dm".to_string());

    // Get the run number parameter (optional)
    let run_number_opt = command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "run")
        .and_then(|opt| opt.value.as_i64());

    // Get the run
    let (run, run_number) = if let Some(num) = run_number_opt {
        match db::get_bet_run_by_number(db, &guild_id, num) {
            Ok(Some(run)) => (run, num),
            Ok(None) => {
                return send_error(ctx, command, &format!("❌ Run #{} not found.", num)).await;
            }
            Err(e) => {
                error!("Database error: {:?}", e);
                return send_error(ctx, command, "❌ Database error. Please try again.").await;
            }
        }
    } else {
        // Get latest active run
        match db::get_active_run(db, &guild_id) {
            Ok(Some(run)) => {
                let num = db::get_run_number(db, &guild_id, run.id).unwrap_or(Some(1)).unwrap_or(1);
                (run, num)
            }
            Ok(None) => {
                return send_error(
                    ctx,
                    command,
                    "❌ No active runs found. Use `/newrun <amount>` to start one.",
                )
                .await;
            }
            Err(e) => {
                error!("Database error: {:?}", e);
                return send_error(ctx, command, "❌ Database error. Please try again.").await;
            }
        }
    };

    // Get bets for this run
    let bets = db::get_bets_for_run(db, run.id).unwrap_or_default();

    // Build the embed
    let status_emoji = match run.status {
        RunStatus::Active => "🟢",
        RunStatus::Lost => "🔴",
        RunStatus::Completed => "🏆",
    };

    let status_text = match run.status {
        RunStatus::Active => "Active",
        RunStatus::Lost => "Lost",
        RunStatus::Completed => "Completed",
    };

    let mut embed = CreateEmbed::new()
        .title(format!("🎲 Henze Bet Run #{}", run_number))
        .field("Status", format!("{} {}", status_emoji, status_text), true)
        .field("Started By", format!("<@{}>", run.created_by), true)
        .field("Started At", &run.created_at, true)
        .field("Starting Amount", format!("{:.2} DKK", run.starting_amount), true)
        .field("Current Amount", format!("**{:.2} DKK**", run.current_amount), true)
        .field("Total Bets", bets.len().to_string(), true);

    // Calculate profit/loss
    let profit = run.current_amount - run.starting_amount;
    let profit_text = if profit >= 0.0 {
        format!("+{:.2} DKK", profit)
    } else {
        format!("{:.2} DKK", profit)
    };
    embed = embed.field("Profit/Loss", profit_text, true);

    // Add bet history (last 5)
    if !bets.is_empty() {
        let mut history = String::new();
        for bet in bets.iter().rev().take(5) {
            let status_icon = match bet.status {
                BetStatus::Pending => "⏳",
                BetStatus::Won => "✅",
                BetStatus::Lost => "❌",
                BetStatus::Void => "⚠️",
            };
            history.push_str(&format!(
                "{} **{}** → {} @ {:.2} ({:.2} DKK)\n",
                status_icon, bet.event_name, bet.outcome_name, bet.odds, bet.stake
            ));
        }
        embed = embed.field("Recent Bets", history, false);
    }

    // Set color based on status
    embed = embed.color(match run.status {
        RunStatus::Active => 0x00FF00,   // Green
        RunStatus::Lost => 0xFF0000,      // Red
        RunStatus::Completed => 0xFFD700, // Gold
    });

    let response = CreateInteractionResponseMessage::new()
        .embed(embed)
        .ephemeral(true);

    command
        .create_response(&ctx.http, CreateInteractionResponse::Message(response))
        .await
}

/// Send an ephemeral error message.
async fn send_error(
    ctx: &Context,
    command: &CommandInteraction,
    message: &str,
) -> Result<(), serenity::Error> {
    let response = CreateInteractionResponseMessage::new()
        .content(message)
        .ephemeral(true);

    command
        .create_response(&ctx.http, CreateInteractionResponse::Message(response))
        .await
}
