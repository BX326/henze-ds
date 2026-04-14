//! Data models for the web application.

use henze_ds::HenzeInfo;
use rocket::FromForm;
use serde::Serialize;

/// Form parameters for filtering bets.
#[derive(FromForm)]
pub struct FilterParams {
    pub target: Option<f64>,
    pub tolerance: Option<f64>,
    pub sport: Option<String>,
}

/// Information about a single market/outcome.
#[derive(Serialize, Clone)]
pub struct MarketInfo {
    pub market_name: String,
    pub outcome: String,
    pub decimal: f64,
}

/// An event grouped with all its markets.
#[derive(Serialize, Clone)]
pub struct GroupedEvent {
    pub event_id: String,
    pub event_name: String,
    pub event_time: String,
    pub event_url: String,
    pub is_live: bool,
    pub match_minute: Option<i32>,
    pub sport_name: String,
    pub markets: Vec<MarketInfo>,
}

/// A sport option for the dropdown selector.
#[derive(Serialize)]
pub struct SportOption {
    pub id: String,
    pub name: String,
    pub selected: bool,
}

/// Complete context for rendering the bets template.
#[derive(Serialize)]
pub struct BetsContext {
    pub bets: Vec<HenzeInfo>,
    pub grouped_events: Vec<GroupedEvent>,
    pub target: f64,
    pub tolerance: f64,
    pub min_odds: f64,
    pub max_odds: f64,
    pub count: usize,
    pub event_count: usize,
    pub error: Option<String>,
    pub sports: Vec<SportOption>,
    pub selected_sport: String,
}
