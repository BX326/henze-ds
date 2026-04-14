use henze_ds::{HenzeFilter, HenzeInfo};
use rand::seq::SliceRandom;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, CreateEmbed, CreateEmbedFooter, CreateMessage, GatewayIntents, Http};
use serenity::prelude::*;
use std::env;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info, warn};

/// Number of bets to include in the daily message
const BETS_PER_MESSAGE: usize = 3;

// OpenAI API structures
#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f64,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessage,
}

/// Format bets into a numbered list for the AI prompt
fn format_bets_for_ai(bets: &[HenzeInfo]) -> String {
    bets.iter()
        .enumerate()
        .map(|(i, bet)| {
            format!(
                "{}. {} | {} | {} @ {:.2} | {} | {}",
                i + 1,
                bet.event_name,
                bet.sport_name,
                bet.outcome,
                bet.decimal,
                bet.market_name,
                if bet.is_live {
                    format!("LIVE ({}min)", bet.match_minute.unwrap_or(0))
                } else {
                    format!("Starts: {}", bet.event_time)
                }
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Default system prompt (embedded at compile time)
const DEFAULT_SYSTEM_PROMPT: &str = include_str!("../system_prompt.txt");

/// Load the system prompt with priority: env var > file > default
fn load_system_prompt() -> String {
    // 1. Try environment variable (for secrets/deployment)
    if let Ok(prompt) = env::var("SYSTEM_PROMPT") {
        info!("Using system prompt from SYSTEM_PROMPT env var");
        return prompt;
    }
    
    // 2. Try custom file (for local development) - check multiple locations
    for path in &["system_prompt.local.txt", "henze-ds-discord/system_prompt.local.txt"] {
        if let Ok(custom) = std::fs::read_to_string(path) {
            info!("Using custom system prompt from {}", path);
            return custom;
        }
    }
    
    // 3. Fall back to default
    DEFAULT_SYSTEM_PROMPT.to_string()
}

/// Use OpenAI to select the most interesting bets
async fn select_bets_with_ai(
    bets: &[HenzeInfo],
    count: usize,
) -> Result<Vec<usize>, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    
    let bets_list = format_bets_for_ai(bets);
    
    let system_prompt = load_system_prompt();

    let user_prompt = format!(
        "Select the {} most interesting bets from this list:\n\n{}\n\nReturn only the numbers, comma-separated.",
        count,
        bets_list
    );

    let request = OpenAIRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: system_prompt,
            },
            ChatMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ],
        temperature: 0.3,
    };

    let client = HttpClient::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(format!("OpenAI API error: {}", error_text).into());
    }

    let openai_response: OpenAIResponse = response.json().await?;
    
    let content = &openai_response.choices[0].message.content;
    info!("AI selected bets: {}", content);
    
    // Parse the comma-separated numbers
    let indices: Vec<usize> = content
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .filter(|&n| n >= 1 && n <= bets.len())
        .map(|n| n - 1) // Convert to 0-indexed
        .take(count)
        .collect();

    Ok(indices)
}

/// Fetch Henze bets and select the best ones (AI-powered with random fallback)
async fn fetch_best_bets(count: usize) -> Result<Vec<HenzeInfo>, Box<dyn std::error::Error + Send + Sync>> {
    let filter = HenzeFilter::default();
    let bets = henze_ds::retrieve_henze_data_with_filter(filter)
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.to_string().into() })?;
    
    if bets.is_empty() {
        return Ok(vec![]);
    }
    
    // Try AI selection first
    if env::var("OPENAI_API_KEY").is_ok() {
        match select_bets_with_ai(&bets, count).await {
            Ok(indices) if !indices.is_empty() => {
                info!("Using AI-selected bets: {:?}", indices);
                let selected: Vec<HenzeInfo> = indices
                    .into_iter()
                    .filter_map(|i| bets.get(i).cloned())
                    .collect();
                
                if !selected.is_empty() {
                    return Ok(selected);
                }
            }
            Ok(_) => {
                warn!("AI returned empty selection, falling back to random");
            }
            Err(e) => {
                warn!("AI selection failed: {}, falling back to random", e);
            }
        }
    } else {
        info!("OPENAI_API_KEY not set, using random selection");
    }
    
    // Fallback to random selection
    let mut bets = bets;
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
        "Disclaimer: Gambling involves risk. Please bet responsibly."
    ));
    
    embed
}

/// Send the daily bets message to the specified channel
async fn send_daily_bets(http: Arc<Http>, channel_id: ChannelId) {
    info!("Fetching daily Henze bets...");
    
    match fetch_best_bets(BETS_PER_MESSAGE).await {
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
        let bets = fetch_best_bets(3).await;
        assert!(bets.is_ok());
    }
}
