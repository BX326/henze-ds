//! Data models for the web application.

use henze_ds::HenzeInfo;
use rocket::FromForm;
use serde::Serialize;

/// Form parameters for filtering bets.
/// Note: Advanced filters (time, league, live status) are handled client-side.
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
    pub market_type: String,
    pub market_sub_type: String,
    pub outcome: String,
    pub decimal: f64,
}

/// An event grouped with all its markets.
#[derive(Serialize, Clone)]
pub struct GroupedEvent {
    pub event_id: String,
    pub event_name: String,
    pub event_time: String,
    pub event_time_utc: String,
    pub event_url: String,
    pub is_live: bool,
    pub match_minute: Option<i32>,
    pub sport_name: String,
    /// Category ID (league/competition identifier)
    pub category_id: String,
    /// Category name (e.g., "Premier League", "Bundesliga")
    pub category_name: String,
    /// Class ID (country/region identifier)
    pub class_id: String,
    /// Class name (e.g., "England", "Germany")
    pub class_name: String,
    pub markets: Vec<MarketInfo>,
}

/// A sport option for the dropdown selector.
#[derive(Serialize)]
pub struct SportOption {
    pub id: String,
    pub name: String,
    pub selected: bool,
}

/// A filter option for dropdowns (league, country, etc.)
#[derive(Serialize, Clone)]
pub struct FilterOption {
    pub id: String,
    pub name: String,
    pub count: usize,
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
    /// Available categories (leagues) for filtering
    pub categories: Vec<FilterOption>,
    /// Available classes (countries) for filtering
    pub classes: Vec<FilterOption>,
    /// Current time preset selection
    pub time_preset: String,
    /// Custom from time (ISO 8601)
    pub from_time: String,
    /// Custom to time (ISO 8601)
    pub to_time: String,
    /// Live-only filter active
    pub live_only: bool,
}
