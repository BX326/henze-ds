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
}

/// Parameters for filtering Henze bets
pub struct HenzeFilter {
    /// Target odds (default: 1.1)
    pub target: f64,
    /// Tolerance around target (default: 0.04, meaning 1.06 to 1.14)
    pub tolerance: f64,
    /// Sport drilldown tag ID (None = all sports, 12 = football)
    pub sport_tag_id: Option<String>,
}

impl Default for HenzeFilter {
    fn default() -> Self {
        Self {
            target: DEFAULT_TARGET_ODDS,
            tolerance: DEFAULT_TOLERANCE,
            sport_tag_id: None, // All sports by default
        }
    }
}

impl HenzeFilter {
    pub fn new(target: f64, tolerance: f64) -> Self {
        Self { target, tolerance, sport_tag_id: None }
    }

    pub fn with_sport(target: f64, tolerance: f64, sport_tag_id: Option<String>) -> Self {
        Self { target, tolerance, sport_tag_id }
    }

    /// Create a filter that accepts all odds (no filtering)
    pub fn all() -> Self {
        Self {
            target: 500.0,
            tolerance: 500.0,
            sport_tag_id: None,
        }
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

/// Get sport name from category name (e.g., "Football", "Tennis")
fn get_sport_name(category_name: &str) -> String {
    category_name.to_string()
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
            time_band_event.events.iter().flat_map(move |event| {
                let event_id = &event.id;
                let event_name = &event.name;
                let event_time_utc = event.start_time;
                let event_time = format_time_utc_plus_2(&event_time_utc);
                let event_url = format!("https://danskespil.dk/oddset/in-play/event/{}", event_id);
                let is_live = event.live_now;
                let sport_id = event.sport_id.clone();
                let sport_name = get_sport_name(&event.category.name);
                
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
                    market.outcomes.iter().flat_map(move |outcome| {
                        let outcome_name = &outcome.name;
                        let event_url = event_url.clone();
                        let event_time = event_time.clone();
                        let sport_id = sport_id.clone();
                        let sport_name = sport_name.clone();
                        outcome.prices.iter().filter_map(move |price| {
                            (price.decimal >= min_odds && price.decimal <= max_odds).then(|| HenzeInfo {
                                event_id: event_id.clone(),
                                event_name: event_name.clone(),
                                event_time: event_time.clone(),
                                event_time_utc: event_time_utc.to_rfc3339(),
                                market_name: market_name.clone().to_string(),
                                outcome: outcome_name.clone(),
                                decimal: price.decimal,
                                event_url: event_url.clone(),
                                is_live,
                                match_minute,
                                sport_id: sport_id.clone(),
                                sport_name: sport_name.clone(),
                            })
                        })
                    })
                })
            })
        })
        .collect();

    Ok(collected_info)
}