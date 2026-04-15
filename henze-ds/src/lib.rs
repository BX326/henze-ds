use chrono::{DateTime, FixedOffset, Utc};
use serde::Serialize;

pub mod ds_client;

// Re-export available_sports for easy access
pub use ds_client::client::{available_sports, DEFAULT_SPORT};

/// Default target odds for Henze bets
pub const DEFAULT_TARGET_ODDS: f64 = 1.1;

/// Default tolerance around the target odds
pub const DEFAULT_TOLERANCE: f64 = 0.04;

/// UTC+2 offset in seconds
const UTC_PLUS_2: i32 = 2 * 3600;

#[derive(Debug, Clone, Serialize)]
pub struct HenzeInfo {
    pub event_id: String,
    pub event_name: String,
    pub event_time: String,
    pub event_time_utc: String,
    pub market_name: String,
    pub outcome: String,
    pub decimal: f64,
    pub event_url: String,
    pub is_live: bool,
    pub match_minute: Option<i32>,
    pub sport_id: String,
    pub sport_name: String,
    /// Category ID (league/competition identifier)
    pub category_id: String,
    /// Category name (e.g., "Premier League", "Bundesliga")
    pub category_name: String,
    /// Class ID (country/region identifier)
    pub class_id: String,
    /// Class name (e.g., "England", "Germany")
    pub class_name: String,
}

/// Extended bet info including market and outcome IDs for bet placement.
#[derive(Debug, Clone, Serialize)]
pub struct BetOption {
    pub event_id: String,
    pub event_name: String,
    pub event_time: String,
    pub event_url: String,
    pub is_live: bool,
    pub sport_name: String,
    pub category_name: String,
    /// Market ID for this betting option
    pub market_id: String,
    pub market_name: String,
    /// Outcome ID for this betting option
    pub outcome_id: String,
    pub outcome_name: String,
    pub odds: f64,
    /// Whether this outcome has been resulted (settled)
    pub resulted: bool,
    /// Outcome status string (e.g., may indicate win/loss)
    pub status: String,
}

/// Result of checking an outcome's settlement status.
#[derive(Debug, Clone, PartialEq)]
pub enum BetResult {
    /// Bet is still pending (not resulted yet)
    Pending,
    /// Bet won
    Won,
    /// Bet lost
    Lost,
    /// Bet was voided
    Void,
    /// Event/outcome not found (may have been removed from API)
    NotFound,
}

/// Parameters for filtering Henze bets
pub struct HenzeFilter {
    /// Target odds (default: 1.1)
    pub target: f64,
    /// Tolerance around target (default: 0.04, meaning 1.06 to 1.14)
    pub tolerance: f64,
    /// Sport drilldown tag ID (None = all sports, 12 = football)
    pub sport_tag_id: Option<String>,
    /// Filter events starting from this time (inclusive)
    pub start_time_from: Option<DateTime<Utc>>,
    /// Filter events starting until this time (inclusive)
    pub start_time_to: Option<DateTime<Utc>>,
    /// Only include live events
    pub live_only: bool,
}

impl Default for HenzeFilter {
    fn default() -> Self {
        Self {
            target: DEFAULT_TARGET_ODDS,
            tolerance: DEFAULT_TOLERANCE,
            sport_tag_id: None, // All sports by default
            start_time_from: None,
            start_time_to: None,
            live_only: false,
        }
    }
}

impl HenzeFilter {
    pub fn new(target: f64, tolerance: f64) -> Self {
        Self { 
            target, 
            tolerance, 
            sport_tag_id: None,
            start_time_from: None,
            start_time_to: None,
            live_only: false,
        }
    }

    pub fn with_sport(target: f64, tolerance: f64, sport_tag_id: Option<String>) -> Self {
        Self { 
            target, 
            tolerance, 
            sport_tag_id,
            start_time_from: None,
            start_time_to: None,
            live_only: false,
        }
    }
    
    /// Builder method to set time range
    pub fn with_time_range(mut self, from: Option<DateTime<Utc>>, to: Option<DateTime<Utc>>) -> Self {
        self.start_time_from = from;
        self.start_time_to = to;
        self
    }
    
    /// Builder method to set live-only filter
    pub fn with_live_only(mut self, live_only: bool) -> Self {
        self.live_only = live_only;
        self
    }

    pub fn min_odds(&self) -> f64 {
        self.target - self.tolerance
    }

    pub fn max_odds(&self) -> f64 {
        self.target + self.tolerance
    }

    pub fn matches(&self, odds: f64) -> bool {
        odds >= self.min_odds() && odds <= self.max_odds()
    }
    
    /// Check if an event passes time and live filters
    pub fn matches_event(&self, start_time: &DateTime<Utc>, is_live: bool) -> bool {
        // Live filter
        if self.live_only && !is_live {
            return false;
        }
        
        // Time range filter
        if let Some(from) = self.start_time_from {
            if *start_time < from {
                return false;
            }
        }
        if let Some(to) = self.start_time_to {
            if *start_time > to {
                return false;
            }
        }
        
        true
    }
}

pub async fn retrieve_henze_data() -> Result<Vec<HenzeInfo>, Box<dyn std::error::Error>> {
    retrieve_henze_data_with_filter(HenzeFilter::default()).await
}

/// Convert UTC datetime string to UTC+2
fn format_time_utc_plus_2(start_time: &DateTime<Utc>) -> String {
    let offset = FixedOffset::east_opt(UTC_PLUS_2).unwrap();
    let local_time = start_time.with_timezone(&offset);
    local_time.format("%Y-%m-%d %H:%M").to_string()
}

/// Get sport name from class name (e.g., "FOOTBALL", "TENNIS")
fn get_sport_name(class_name: &str) -> String {
    class_name.to_string()
}

pub async fn retrieve_henze_data_with_filter(
    filter: HenzeFilter,
) -> Result<Vec<HenzeInfo>, Box<dyn std::error::Error>> {
    let client = ds_client::client::ApiClient::new();
    let data = client.get_data_with_sport(filter.sport_tag_id.as_deref()).await?;

    let min_odds = filter.min_odds();
    let max_odds = filter.max_odds();

    let collected_info: Vec<HenzeInfo> = data
        .data
        .time_band_events
        .iter()
        .flat_map(|time_band_event| {
            time_band_event.events.iter()
                // Apply time and live filters at event level
                .filter(|event| filter.matches_event(&event.start_time, event.live_now))
                .flat_map(move |event| {
                let event_id = &event.id;
                let event_name = &event.name;
                let event_time_utc = event.start_time;
                let event_time = format_time_utc_plus_2(&event_time_utc);
                let event_url = format!("https://danskespil.dk/oddset/in-play/event/{}", event_id);
                let is_live = event.live_now;
                let sport_id = event.sport_id.clone();
                let sport_name = get_sport_name(&event.class.name);
                
                // Category = league/competition (e.g., "Premier League")
                let category_id = event.category.id.clone();
                let category_name = event.category.name.clone();
                
                // Class = country/region (e.g., "England")
                let class_id = event.class.id.clone();
                let class_name = event.class.name.clone();
                
                // Get match minute from commentary clock if available
                let match_minute = event.commentary.as_ref().and_then(|c| {
                    c.clock.as_ref().map(|clock| (clock.offset / 60) as i32)
                });

                event.markets.iter().flat_map(move |market| {
                    let market_name = &market.name;
                    let event_url = event_url.clone();
                    let event_time = event_time.clone();
                    let sport_id = sport_id.clone();
                    let sport_name = sport_name.clone();
                    let category_id = category_id.clone();
                    let category_name = category_name.clone();
                    let class_id = class_id.clone();
                    let class_name = class_name.clone();
                    market.outcomes.iter().flat_map(move |outcome| {
                        let outcome_name = &outcome.name;
                        let event_url = event_url.clone();
                        let event_time = event_time.clone();
                        let sport_id = sport_id.clone();
                        let sport_name = sport_name.clone();
                        let category_id = category_id.clone();
                        let category_name = category_name.clone();
                        let class_id = class_id.clone();
                        let class_name = class_name.clone();
                        outcome.prices.iter().filter_map(move |price| {
                            // Handle optional decimal - skip if None
                            let decimal = price.decimal?;
                            (decimal >= min_odds && decimal <= max_odds).then(|| HenzeInfo {
                                event_id: event_id.clone(),
                                event_name: event_name.clone(),
                                event_time: event_time.clone(),
                                event_time_utc: event_time_utc.to_rfc3339(),
                                market_name: market_name.clone().to_string(),
                                outcome: outcome_name.clone(),
                                decimal,
                                event_url: event_url.clone(),
                                is_live,
                                match_minute,
                                sport_id: sport_id.clone(),
                                sport_name: sport_name.clone(),
                                category_id: category_id.clone(),
                                category_name: category_name.clone(),
                                class_id: class_id.clone(),
                                class_name: class_name.clone(),
                            })
                        })
                    })
                })
            })
        })
        .collect();

    Ok(collected_info)
}

/// Get all betting options for a specific event by ID.
/// Returns all outcomes regardless of odds (for selecting which bet to place).
pub async fn get_event_bet_options(event_id: &str) -> Result<Vec<BetOption>, Box<dyn std::error::Error + Send + Sync>> {
    let client = ds_client::client::ApiClient::new();
    // Fetch all events (no sport filter to ensure we find the event)
    let data = client.get_data_with_sport(None).await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.to_string().into() })?;

    let mut options = Vec::new();

    for time_band_event in &data.data.time_band_events {
        for event in &time_band_event.events {
            if event.id != event_id {
                continue;
            }

            let event_time = format_time_utc_plus_2(&event.start_time);
            let event_url = format!("https://danskespil.dk/oddset/in-play/event/{}", event.id);

            for market in &event.markets {
                for outcome in &market.outcomes {
                    // Get the best price for this outcome
                    if let Some(price) = outcome.prices.first() {
                        if let Some(odds) = price.decimal {
                            options.push(BetOption {
                                event_id: event.id.clone(),
                                event_name: event.name.clone(),
                                event_time: event_time.clone(),
                                event_url: event_url.clone(),
                                is_live: event.live_now,
                                sport_name: event.class.name.clone(),
                                category_name: event.category.name.clone(),
                                market_id: market.id.clone(),
                                market_name: market.name.to_string(),
                                outcome_id: outcome.id.clone(),
                                outcome_name: outcome.name.clone(),
                                odds,
                                resulted: outcome.resulted,
                                status: outcome.status.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(options)
}

/// Get Henze-eligible betting options for a specific event (odds ~1.06-1.14).
pub async fn get_henze_options_for_event(event_id: &str) -> Result<Vec<BetOption>, Box<dyn std::error::Error + Send + Sync>> {
    let all_options = get_event_bet_options(event_id).await?;
    let filter = HenzeFilter::default();
    
    Ok(all_options
        .into_iter()
        .filter(|opt| filter.matches(opt.odds))
        .collect())
}

/// Get a specific betting option by event ID and outcome ID.
pub async fn get_bet_option(event_id: &str, outcome_id: &str) -> Result<Option<BetOption>, Box<dyn std::error::Error + Send + Sync>> {
    let options = get_event_bet_options(event_id).await?;
    Ok(options.into_iter().find(|opt| opt.outcome_id == outcome_id))
}

/// Check the result of a specific outcome for bet settlement.
/// 
/// This checks if the outcome has been resulted and attempts to determine
/// if the bet won or lost based on the status field.
pub async fn check_outcome_result(event_id: &str, outcome_id: &str) -> Result<BetResult, Box<dyn std::error::Error + Send + Sync>> {
    let option = get_bet_option(event_id, outcome_id).await?;
    
    match option {
        None => Ok(BetResult::NotFound),
        Some(opt) => {
            if !opt.resulted {
                return Ok(BetResult::Pending);
            }
            
            // Parse the status string to determine win/loss/void
            // Common status values are typically: "W" (win), "L" (lose), "V" (void), "S" (settled)
            // The exact values may vary - this is a best-effort interpretation
            let status_upper = opt.status.to_uppercase();
            
            if status_upper.contains("WIN") || status_upper == "W" || status_upper == "WINNER" {
                Ok(BetResult::Won)
            } else if status_upper.contains("LOSE") || status_upper.contains("LOST") || status_upper == "L" || status_upper == "LOSER" {
                Ok(BetResult::Lost)
            } else if status_upper.contains("VOID") || status_upper == "V" || status_upper.contains("CANCEL") {
                Ok(BetResult::Void)
            } else if status_upper.contains("SUSPEND") || status_upper == "S" {
                // Suspended outcomes are still pending
                Ok(BetResult::Pending)
            } else {
                // If resulted but status unclear, treat as pending
                // This allows for manual inspection of unknown status values
                eprintln!(
                    "Warning: Unknown outcome status '{}' for event {} outcome {} - treating as pending",
                    opt.status, event_id, outcome_id
                );
                Ok(BetResult::Pending)
            }
        }
    }
}