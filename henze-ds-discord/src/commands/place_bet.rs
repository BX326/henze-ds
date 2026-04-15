//! /placebet command - Place a bet within a Henze bet run.

use henze_ds::{get_bet_option, get_henze_options_for_event, BetOption, HenzeFilter};
use serenity::all::{
    CommandInteraction, CommandOptionType, ComponentInteraction, Context, CreateCommand,
    CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption, CreateActionRow,
};
use tracing::{error, info};

use crate::config::get_announcement_channel_id;
use crate::db::{self, DbPool, RunStatus};
use crate::discord::announce_bet_placed;

/// Create the command registration.
pub fn register() -> CreateCommand {
    CreateCommand::new("placebet")
        .description("Place a bet within a Henze bet run")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "run",
                "Run number (e.g., 1 for Run #1)",
            )
            .required(true)
            .min_int_value(1),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "event_id",
                "Danske Spil event ID",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "outcome_id",
                "Specific outcome ID (optional - shows selection menu if omitted)",
            )
            .required(false),
        )
}

/// Execute the /placebet command.
pub async fn place_bet(ctx: &Context, command: &CommandInteraction, db: &DbPool) -> Result<(), serenity::Error> {
    let guild_id = command
        .guild_id
        .map(|g| g.to_string())
        .unwrap_or_else(|| "dm".to_string());

    // Get parameters
    let run_number = command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "run")
        .and_then(|opt| opt.value.as_i64())
        .unwrap_or(1);

    let event_id = command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "event_id")
        .and_then(|opt| opt.value.as_str())
        .unwrap_or("")
        .to_string();

    let outcome_id = command
        .data
        .options
        .iter()
        .find(|opt| opt.name == "outcome_id")
        .and_then(|opt| opt.value.as_str())
        .map(|s| s.to_string());

    // Validate run exists and is active
    let run = match db::get_bet_run_by_number(db, &guild_id, run_number) {
        Ok(Some(run)) => run,
        Ok(None) => {
            return send_error(ctx, command, &format!("❌ Run #{} not found.", run_number)).await;
        }
        Err(e) => {
            error!("Database error: {:?}", e);
            return send_error(ctx, command, "❌ Database error. Please try again.").await;
        }
    };

    if run.status != RunStatus::Active {
        return send_error(
            ctx,
            command,
            &format!("❌ Run #{} is not active (status: {:?}).", run_number, run.status),
        )
        .await;
    }

    // Check for pending bet
    match db::has_pending_bet(db, run.id) {
        Ok(true) => {
            return send_error(
                ctx,
                command,
                &format!(
                    "❌ Run #{} already has a pending bet. Wait for it to settle before placing another.",
                    run_number
                ),
            )
            .await;
        }
        Ok(false) => {}
        Err(e) => {
            error!("Database error checking pending bet: {:?}", e);
            return send_error(ctx, command, "❌ Database error. Please try again.").await;
        }
    }

    // If outcome_id is provided, place the bet directly
    if let Some(ref oid) = outcome_id {
        return place_bet_with_outcome(ctx, command, db, &run, &event_id, oid, run_number).await;
    }

    // Otherwise, fetch available Henze options and show selection
    command.defer_ephemeral(&ctx.http).await?;

    match get_henze_options_for_event(&event_id).await {
        Ok(options) if options.is_empty() => {
            command
                .edit_response(
                    &ctx.http,
                    serenity::all::EditInteractionResponse::new()
                        .content(format!(
                            "❌ No Henze bets available for event `{}` (no outcomes with odds ~1.06-1.14).",
                            event_id
                        )),
                )
                .await?;
        }
        Ok(options) if options.len() == 1 => {
            // Auto-select single option
            let opt = &options[0];
            place_bet_internal(ctx, command, db, &run, opt, run_number).await?;
        }
        Ok(options) => {
            // Show selection menu
            show_outcome_selection(ctx, command, &run, &options, run_number).await?;
        }
        Err(e) => {
            error!("Failed to fetch event options: {:?}", e);
            command
                .edit_response(
                    &ctx.http,
                    serenity::all::EditInteractionResponse::new()
                        .content(format!(
                            "❌ Failed to fetch event `{}`. It may not exist or the API is unavailable.",
                            event_id
                        )),
                )
                .await?;
        }
    }

    Ok(())
}

/// Place a bet with a specific outcome ID.
async fn place_bet_with_outcome(
    ctx: &Context,
    command: &CommandInteraction,
    db: &DbPool,
    run: &db::BetRun,
    event_id: &str,
    outcome_id: &str,
    run_number: i64,
) -> Result<(), serenity::Error> {
    command.defer_ephemeral(&ctx.http).await?;

    // Fetch the specific outcome
    match get_bet_option(event_id, outcome_id).await {
        Ok(Some(option)) => {
            // Validate it's a Henze bet
            let filter = HenzeFilter::default();
            if !filter.matches(option.odds) {
                command
                    .edit_response(
                        &ctx.http,
                        serenity::all::EditInteractionResponse::new()
                            .content(format!(
                                "❌ Outcome `{}` has odds {:.2}, which is outside Henze range (1.06-1.14).",
                                outcome_id, option.odds
                            )),
                    )
                    .await?;
                return Ok(());
            }

            place_bet_internal(ctx, command, db, run, &option, run_number).await?;
        }
        Ok(None) => {
            command
                .edit_response(
                    &ctx.http,
                    serenity::all::EditInteractionResponse::new()
                        .content(format!(
                            "❌ Outcome `{}` not found for event `{}`.",
                            outcome_id, event_id
                        )),
                )
                .await?;
        }
        Err(e) => {
            error!("Failed to fetch bet option: {:?}", e);
            command
                .edit_response(
                    &ctx.http,
                    serenity::all::EditInteractionResponse::new()
                        .content("❌ Failed to fetch event data. Please try again."),
                )
                .await?;
        }
    }

    Ok(())
}

/// Internal function to actually place the bet and announce it.
async fn place_bet_internal(
    ctx: &Context,
    command: &CommandInteraction,
    db: &DbPool,
    run: &db::BetRun,
    option: &BetOption,
    run_number: i64,
) -> Result<(), serenity::Error> {
    let user_id = command.user.id.to_string();
    let stake = run.current_amount;
    let potential_return = stake * option.odds;

    // Create the bet in database
    match db::create_bet(
        db,
        run.id,
        &option.event_id,
        &option.outcome_id,
        &option.market_id,
        &option.event_name,
        &option.market_name,
        &option.outcome_name,
        option.odds,
        stake,
        &user_id,
    ) {
        Ok(bet) => {
            info!(
                "Created bet {} for run {} on event {} outcome {}",
                bet.id, run.id, option.event_id, option.outcome_id
            );

            // Send confirmation
            command
                .edit_response(
                    &ctx.http,
                    serenity::all::EditInteractionResponse::new()
                        .content(format!(
                            "✅ **Bet Placed!**\n\n\
                            **Run #{}** | Stake: **{:.2} DKK**\n\
                            **Event:** {}\n\
                            **Market:** {}\n\
                            **Outcome:** {} @ **{:.2}**\n\
                            **Potential Return:** {:.2} DKK\n\n\
                            🎯 Waiting for result...",
                            run_number,
                            stake,
                            option.event_name,
                            option.market_name,
                            option.outcome_name,
                            option.odds,
                            potential_return
                        )),
                )
                .await?;

            // Announce in the configured announcement channel
            if let Some(channel_id) = get_announcement_channel_id() {
                let _ = announce_bet_placed(
                    &ctx.http,
                    serenity::all::ChannelId::new(channel_id),
                    run_number,
                    &bet,
                    option,
                )
                .await;
            }
        }
        Err(e) => {
            error!("Failed to create bet: {:?}", e);
            command
                .edit_response(
                    &ctx.http,
                    serenity::all::EditInteractionResponse::new()
                        .content("❌ Failed to save bet. Please try again."),
                )
                .await?;
        }
    }

    Ok(())
}

/// Show a selection menu for multiple Henze options.
async fn show_outcome_selection(
    ctx: &Context,
    command: &CommandInteraction,
    run: &db::BetRun,
    options: &[BetOption],
    run_number: i64,
) -> Result<(), serenity::Error> {
    let menu_options: Vec<CreateSelectMenuOption> = options
        .iter()
        .take(25) // Discord limit
        .map(|opt| {
            CreateSelectMenuOption::new(
                format!("{} → {} @ {:.2}", opt.market_name, opt.outcome_name, opt.odds),
                format!("{}:{}", opt.event_id, opt.outcome_id),
            )
            .description(format!("Potential: {:.2} DKK", run.current_amount * opt.odds))
        })
        .collect();

    let select_menu = CreateSelectMenu::new(
        format!("placebet_select:{}:{}", run.id, run_number),
        CreateSelectMenuKind::String { options: menu_options },
    )
    .placeholder("Choose an outcome...");

    let action_row = CreateActionRow::SelectMenu(select_menu);

    command
        .edit_response(
            &ctx.http,
            serenity::all::EditInteractionResponse::new()
                .content(format!(
                    "🎯 **Multiple Henze bets found for {}**\n\nSelect an outcome:",
                    options.first().map(|o| o.event_name.as_str()).unwrap_or("event")
                ))
                .components(vec![action_row]),
        )
        .await?;

    Ok(())
}

/// Handle the select menu interaction for outcome selection.
pub async fn handle_outcome_selection(
    ctx: &Context,
    interaction: &ComponentInteraction,
    db: &DbPool,
) -> Result<(), serenity::Error> {
    // Parse custom_id: "placebet_select:run_id:run_number"
    let parts: Vec<&str> = interaction.data.custom_id.split(':').collect();
    if parts.len() != 3 {
        return Ok(());
    }

    let run_id: i64 = parts[1].parse().unwrap_or(0);
    let run_number: i64 = parts[2].parse().unwrap_or(0);

    // Get selected value: "event_id:outcome_id"
    let selected = match &interaction.data.kind {
        serenity::all::ComponentInteractionDataKind::StringSelect { values } => {
            values.first().cloned()
        }
        _ => None,
    };

    let Some(selected) = selected else {
        return Ok(());
    };

    let value_parts: Vec<&str> = selected.split(':').collect();
    if value_parts.len() != 2 {
        return Ok(());
    }

    let event_id = value_parts[0];
    let outcome_id = value_parts[1];

    // Get the run
    let run = match db::get_bet_run_by_id(db, run_id) {
        Ok(Some(run)) => run,
        _ => {
            let response = CreateInteractionResponseMessage::new()
                .content("❌ Run not found.")
                .ephemeral(true);
            interaction
                .create_response(&ctx.http, CreateInteractionResponse::Message(response))
                .await?;
            return Ok(());
        }
    };

    // Defer the interaction (ephemeral)
    interaction.defer_ephemeral(&ctx.http).await?;

    // Fetch the option and place the bet
    match get_bet_option(event_id, outcome_id).await {
        Ok(Some(option)) => {
            let user_id = interaction.user.id.to_string();
            let stake = run.current_amount;
            let potential_return = stake * option.odds;

            match db::create_bet(
                db,
                run.id,
                &option.event_id,
                &option.outcome_id,
                &option.market_id,
                &option.event_name,
                &option.market_name,
                &option.outcome_name,
                option.odds,
                stake,
                &user_id,
            ) {
                Ok(bet) => {
                    info!(
                        "Created bet {} for run {} via selection",
                        bet.id, run.id
                    );

                    interaction
                        .edit_response(
                            &ctx.http,
                            serenity::all::EditInteractionResponse::new()
                                .content(format!(
                                    "✅ **Bet Placed!**\n\n\
                                    **Run #{}** | Stake: **{:.2} DKK**\n\
                                    **Event:** {}\n\
                                    **Market:** {}\n\
                                    **Outcome:** {} @ **{:.2}**\n\
                                    **Potential Return:** {:.2} DKK\n\n\
                                    🎯 Waiting for result...",
                                    run_number,
                                    stake,
                                    option.event_name,
                                    option.market_name,
                                    option.outcome_name,
                                    option.odds,
                                    potential_return
                                ))
                                .components(vec![]), // Remove the select menu
                        )
                        .await?;

                    // Announce in the configured announcement channel
                    if let Some(channel_id) = get_announcement_channel_id() {
                        let _ = announce_bet_placed(
                            &ctx.http,
                            serenity::all::ChannelId::new(channel_id),
                            run_number,
                            &bet,
                            &option,
                        )
                        .await;
                    }
                }
                Err(e) => {
                    error!("Failed to create bet: {:?}", e);
                    interaction
                        .edit_response(
                            &ctx.http,
                            serenity::all::EditInteractionResponse::new()
                                .content("❌ Failed to save bet. Please try again.")
                                .components(vec![]),
                        )
                        .await?;
                }
            }
        }
        _ => {
            interaction
                .edit_response(
                    &ctx.http,
                    serenity::all::EditInteractionResponse::new()
                        .content("❌ Failed to fetch bet option. Please try again.")
                        .components(vec![]),
                )
                .await?;
        }
    }

    Ok(())
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
