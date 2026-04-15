//! Discord message formatting and sending utilities.

use henze_ds::{BetOption, HenzeInfo};
use serenity::all::{ChannelId, CreateEmbed, CreateEmbedFooter, CreateMessage, Http};
use std::sync::Arc;
use tracing::{error, info};

use crate::config::BETS_PER_MESSAGE;
use crate::db::{Bet, BetRun, BetStatus};
use crate::openai::fetch_best_bets;

/// Format a single bet as an embed field.
fn format_bet(bet: &HenzeInfo) -> (String, String, bool) {
    let title = format!("⚽ {}", bet.event_name);
    let description = format!(
        "**Market:** {}\n**Outcome:** {}\n**Odds:** {:.2}\n**Time:** {}\n[View Event]({})",
        bet.market_name, bet.outcome, bet.decimal, bet.event_time, bet.event_url
    );
    (title, description, false)
}

/// Create the daily bets embed message.
fn create_bets_embed(bets: &[HenzeInfo]) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title("🎰 Daily Henze Picks")
        .description(format!(
            "Here are today's {} curated Henze bets (odds ~1.10):",
            bets.len()
        ))
        .color(0x00FF00);

    for bet in bets {
        let (title, description, inline) = format_bet(bet);
        embed = embed.field(title, description, inline);
    }

    embed = embed.footer(CreateEmbedFooter::new(
        "Disclaimer: Gambling involves risk. Please bet responsibly.",
    ));

    embed
}

/// Send the daily bets message to the specified channel.
pub async fn send_daily_bets(http: Arc<Http>, channel_id: ChannelId) {
    info!("Fetching daily Henze bets...");

    match fetch_best_bets(BETS_PER_MESSAGE).await {
        Ok(bets) if bets.is_empty() => {
            info!("No bets found for today");
            let message =
                CreateMessage::new().content("No Henze bets available today. Check back tomorrow!");
            if let Err(e) = channel_id.send_message(&http, message).await {
                error!("Failed to send message: {:?}", e);
            }
        }
        Ok(bets) => {
            info!("Found {} bets, sending to channel", bets.len());
            let embed = create_bets_embed(&bets);
            let message = CreateMessage::new().embed(embed);
            if let Err(e) = channel_id.send_message(&http, message).await {
                error!("Failed to send message: {:?}", e);
            }
        }
        Err(e) => {
            error!("Failed to fetch bets: {:?}", e);
            let message = CreateMessage::new()
                .content("Failed to fetch today's bets. Please try again later.");
            if let Err(e) = channel_id.send_message(&http, message).await {
                error!("Failed to send error message: {:?}", e);
            }
        }
    }
}

/// Announce a bet that was just placed.
pub async fn announce_bet_placed(
    http: &Http,
    channel_id: ChannelId,
    run_number: i64,
    bet: &Bet,
    option: &BetOption,
) -> Result<(), serenity::Error> {
    let embed = CreateEmbed::new()
        .title(format!("🎯 Bet Placed - Run #{}", run_number))
        .description(format!(
            "**{}**\n{} • {}",
            option.event_name, option.sport_name, option.category_name
        ))
        .field("Market", &option.market_name, true)
        .field("Outcome", &option.outcome_name, true)
        .field("Odds", format!("{:.2}", option.odds), true)
        .field("Stake", format!("{:.2} DKK", bet.stake), true)
        .field("Potential Return", format!("{:.2} DKK", bet.potential_return), true)
        .field("Placed By", format!("<@{}>", bet.placed_by), true)
        .url(&option.event_url)
        .color(0x3498DB) // Blue
        .footer(CreateEmbedFooter::new("Waiting for result..."));

    let message = CreateMessage::new().embed(embed);
    channel_id.send_message(http, message).await?;
    Ok(())
}

/// Announce the result of a bet.
pub async fn announce_bet_result(
    http: &Http,
    channel_id: ChannelId,
    run_number: i64,
    bet: &Bet,
    run: &BetRun,
    result: BetStatus,
) -> Result<(), serenity::Error> {
    let (title, description, color) = match result {
        BetStatus::Won => {
            let new_amount = bet.stake * bet.odds;
            (
                format!("✅ Bet Won! - Run #{}", run_number),
                format!(
                    "**{}** won!\n\n\
                    {} → {} @ {:.2}\n\n\
                    **Stake:** {:.2} DKK\n\
                    **Winnings:** {:.2} DKK\n\
                    **New Balance:** {:.2} DKK\n\n\
                    🎰 Ready for the next bet!",
                    bet.event_name,
                    bet.market_name,
                    bet.outcome_name,
                    bet.odds,
                    bet.stake,
                    new_amount,
                    new_amount
                ),
                0x2ECC71, // Green
            )
        }
        BetStatus::Lost => {
            (
                format!("❌ Bet Lost! - Run #{}", run_number),
                format!(
                    "**{}** lost.\n\n\
                    {} → {} @ {:.2}\n\n\
                    **Stake Lost:** {:.2} DKK\n\
                    **Final Balance:** 0 DKK\n\n\
                    💔 Run #{} has ended.",
                    bet.event_name,
                    bet.market_name,
                    bet.outcome_name,
                    bet.odds,
                    bet.stake,
                    run_number
                ),
                0xE74C3C, // Red
            )
        }
        BetStatus::Void => (
            format!("⚠️ Bet Voided - Run #{}", run_number),
            format!(
                "**{}** was voided.\n\n\
                {} → {} @ {:.2}\n\n\
                **Stake Refunded:** {:.2} DKK\n\
                **Current Balance:** {:.2} DKK\n\n\
                Place a new bet to continue!",
                bet.event_name,
                bet.market_name,
                bet.outcome_name,
                bet.odds,
                bet.stake,
                run.current_amount
            ),
            0xF39C12, // Orange
        ),
        BetStatus::Pending => {
            // Shouldn't announce pending, but handle it anyway
            return Ok(());
        }
    };

    let embed = CreateEmbed::new()
        .title(title)
        .description(description)
        .color(color)
        .field("Placed By", format!("<@{}>", bet.placed_by), true);

    let message = CreateMessage::new().embed(embed);
    channel_id.send_message(http, message).await?;
    Ok(())
}
