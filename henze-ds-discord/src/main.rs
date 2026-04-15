//! Henze Discord Bot - sends daily curated betting picks to a Discord channel.
//!
//! # Usage
//! - `--now`: Send daily bets immediately (for testing)
//! - `--test-ai`: Test AI selection without Discord (prints results)
//!
//! # Environment Variables
//! - `DISCORD_TOKEN`: Discord bot token (required)
//! - `DISCORD_CHANNEL_ID`: Target channel ID (required)
//! - `OPENAI_API_KEY`: OpenAI API key (optional, enables AI selection)
//! - `CRON_SCHEDULE`: Cron expression for scheduling (default: "0 0 8 * * *")
//! - `SYSTEM_PROMPT`: Custom system prompt for AI (optional)
//! - `DATABASE_PATH`: Path to SQLite database (default: "henze_bets.db")
//! - `SETTLEMENT_INTERVAL_SECS`: Settlement check interval (default: 300)

mod commands;
mod config;
mod db;
mod discord;
mod openai;
mod settlement;

use henze_ds::HenzeFilter;
use serenity::all::{ChannelId, GatewayIntents, Http};
use serenity::prelude::*;
use std::env;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

use config::{get_database_path, get_settlement_interval_secs, BETS_PER_MESSAGE};
use discord::{send_daily_bets, DbPoolKey, Handler};
use openai::fetch_best_bets;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("henze_ds_discord=info".parse().unwrap()),
        )
        .init();

    // Check for flags
    let args: Vec<String> = env::args().collect();
    let send_now = args.iter().any(|arg| arg == "--now");
    let test_ai = args.iter().any(|arg| arg == "--test-ai");

    // Test AI selection without Discord (just prints results)
    if test_ai {
        info!("Testing AI selection...");
        
        // First show how many bets are available
        let filter = HenzeFilter::default();
        let all_bets = henze_ds::retrieve_henze_data_with_filter(filter)
            .await
            .map_err(|e| -> Box<dyn std::error::Error> { e.to_string().into() })?;
        println!("\n📊 Found {} bets in odds range 1.06-1.14\n", all_bets.len());
        
        match fetch_best_bets(BETS_PER_MESSAGE).await {
            Ok(bets) if bets.is_empty() => {
                println!("No bets found.");
            }
            Ok(bets) => {
                println!("\n🎰 AI-Selected Bets:\n");
                for (i, bet) in bets.iter().enumerate() {
                    println!("{}. {} ({}):", i + 1, bet.event_name, bet.sport_name);
                    println!("   Market: {}", bet.market_name);
                    println!("   Outcome: {} @ {:.2}", bet.outcome, bet.decimal);
                    println!("   Time: {}", bet.event_time);
                    println!("   Live: {}", if bet.is_live { "Yes" } else { "No" });
                    println!("   URL: {}\n", bet.event_url);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        return Ok(());
    }

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

    // Initialize database
    let db_path = get_database_path();
    let db = db::init_db(&db_path).expect("Failed to initialize database");
    info!("Database initialized at {:?}", db_path);

    // Create the Discord client with database in context
    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;

    // Store database pool in client data
    {
        let mut data = client.data.write().await;
        data.insert::<DbPoolKey>(db.clone());
    }

    // Set up the scheduler for daily bets
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

    info!("Scheduler started.");

    // Start settlement checker in background
    let settlement_interval = get_settlement_interval_secs();
    let http_for_settlement = http.clone();
    let db_for_settlement = db.clone();
    tokio::spawn(async move {
        settlement::start_settlement_checker(http_for_settlement, db_for_settlement, settlement_interval).await;
    });

    info!("Settlement checker started with {}s interval", settlement_interval);
    info!("Bot is running...");

    // Run the Discord client
    if let Err(e) = client.start().await {
        error!("Client error: {:?}", e);
    }

    Ok(())
}
