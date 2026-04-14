//! OpenAI API integration for AI-powered bet selection.

use henze_ds::{HenzeFilter, HenzeInfo};
use rand::seq::SliceRandom;
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{info, warn};

use crate::config::load_system_prompt;

/// OpenAI chat completion request.
#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f64,
}

/// A single message in the chat conversation.
#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

/// OpenAI chat completion response.
#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

/// A single choice from the OpenAI response.
#[derive(Deserialize)]
struct Choice {
    message: ChatMessage,
}

/// Format bets into a numbered list for the AI prompt.
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

/// Use OpenAI to select the most interesting bets.
async fn select_bets_with_ai(
    bets: &[HenzeInfo],
    count: usize,
) -> Result<Vec<usize>, Box<dyn std::error::Error + Send + Sync>> {
    let api_key = env::var("OPENAI_API_KEY")?;

    let bets_list = format_bets_for_ai(bets);

    let system_prompt = load_system_prompt();

    let user_prompt = format!(
        "Select the {} most interesting bets from this list:\n\n{}\n\nReturn only the numbers, comma-separated.",
        count, bets_list
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

/// Fetch Henze bets and select the best ones (AI-powered with random fallback).
pub async fn fetch_best_bets(
    count: usize,
) -> Result<Vec<HenzeInfo>, Box<dyn std::error::Error + Send + Sync>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_bets() {
        let bets = fetch_best_bets(3).await;
        assert!(bets.is_ok());
    }
}
