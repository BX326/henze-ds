use henze_ds::{HenzeFilter, HenzeInfo};
use rand::seq::SliceRandom;
use serenity::all::{ChannelId, CreateEmbed, CreateEmbedFooter, CreateMessage, GatewayIntents, Http};
use serenity::prelude::*;
use std::env;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

/// Number of bets to include in the daily message
const BETS_PER_MESSAGE: usize = 3;

/// Fetch Henze bets and return up to `count` random ones
async fn fetch_random_bets(count: usize) -> Result<Vec<HenzeInfo>, Box<dyn std::error::Error + Send + Sync>> {
    let filter = HenzeFilter::default();
    let mut bets = henze_ds::retrieve_henze_data_with_filter(filter)
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.to_string().into() })?;
    
    // Shuffle and take up to `count` bets
    let mut rng = rand::thread_rng();
    bets.shuffle(&mut rng);
    bets.truncate(count);
    
    Ok(bets)
}

/// Format a single bet as an embed field
fn format_bet(bet: &HenzeInfo) -> (String, String, bool) {
    let title = format!("⚽ {}", bet.event_name);
    let description = format!(
        "**Market:** {}\n**Outcome:** {}\n**Odds:** {:.2}\n**Time:** {}\n[View Event]({})",
        bet.market_name,
        bet.outcome,
        bet.decimal,
        bet.event_time,
        bet.event_url
    );
    (title, description, false)
}

/// Create the daily bets embed message
fn create_bets_embed(bets: &[HenzeInfo]) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title("🎰 Daily Henze Bets")
        .description(format!(
            "Here are today's {} Henze bets with odds around 1.10 (±0.04):",
            bets.len()
        ))
        .color(0x00FF00);
    
    for bet in bets {
        let (title, description, inline) = format_bet(bet);
        embed = embed.field(title, description, inline);
    }
    
    embed = embed.footer(CreateEmbedFooter::new(
        "Disclaimer: Gambling involves risk. Please bet responsibly."
    ));
    
    embed
}

/// Send the daily bets message to the specified channel
async fn send_daily_bets(http: Arc<Http>, channel_id: ChannelId) {
    info!("Fetching daily Henze bets...");
    
    match fetch_random_bets(BETS_PER_MESSAGE).await {
        Ok(bets) if bets.is_empty() => {
            info!("No bets found for today");
            let message = CreateMessage::new().content("No Henze bets available today. Check back tomorrow!");
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
            let message = CreateMessage::new().content("Failed to fetch today's bets. Please try again later.");
            if let Err(e) = channel_id.send_message(&http, message).await {
                error!("Failed to send error message: {:?}", e);
            }
        }
    }
}

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: serenity::model::gateway::Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("henze_ds_discord=info".parse().unwrap()),
        )
        .init();

    // Check for --now flag (send immediately and exit)
    let args: Vec<String> = env::args().collect();
    let send_now = args.iter().any(|arg| arg == "--now");

    // Load configuration from environment
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");
    let channel_id: u64 = env::var("DISCORD_CHANNEL_ID")
        .expect("DISCORD_CHANNEL_ID must be set")
        .parse()
        .expect("DISCORD_CHANNEL_ID must be a valid u64");
    
    // Schedule cron expression - default to 8:00 AM UTC daily
    let cron_schedule = env::var("CRON_SCHEDULE").unwrap_or_else(|_| "0 0 8 * * *".to_string());

    info!("Starting Henze Discord Bot...");
    info!("Channel ID: {}", channel_id);

    // Create HTTP client for sending messages
    let http = Arc::new(Http::new(&token));
    let channel = ChannelId::new(channel_id);

    if send_now {
        // Send message immediately and exit (useful for testing)
        info!("--now flag detected, sending daily bets immediately...");
        send_daily_bets(http, channel).await;
        info!("Done!");
        return Ok(());
    }

    info!("Cron schedule: {}", cron_schedule);

    // Create the Discord client
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;
    let client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;

    // Set up the scheduler
    let scheduler = JobScheduler::new().await?;
    
    let http_clone = http.clone();
    let job = Job::new_async(cron_schedule.as_str(), move |_uuid, _lock| {
        let http = http_clone.clone();
        Box::pin(async move {
            send_daily_bets(http, channel).await;
        })
    })?;
    
    scheduler.add(job).await?;
    scheduler.start().await?;

    info!("Scheduler started. Bot is running...");

    // Run the Discord client
    let mut client = client;
    if let Err(e) = client.start().await {
        error!("Client error: {:?}", e);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_bets() {
        let bets = fetch_random_bets(3).await;
        assert!(bets.is_ok());
    }
}
