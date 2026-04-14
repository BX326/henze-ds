//! Discord message formatting and sending utilities.

use henze_ds::HenzeInfo;
use serenity::all::{ChannelId, CreateEmbed, CreateEmbedFooter, CreateMessage, Http};
use std::sync::Arc;
use tracing::{error, info};

use crate::config::BETS_PER_MESSAGE;
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
