//! Data models for the web application.

use serde::Serialize;

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
    pub category_id: String,
    pub category_name: String,
    pub class_id: String,
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

/// Thin shell context for the HTML page — no event data, just form defaults.
#[derive(Serialize)]
pub struct ShellContext {
    pub target: f64,
    pub tolerance: f64,
    pub min_odds: f64,
    pub max_odds: f64,
    pub sports: Vec<SportOption>,
    pub selected_sport: String,
}

/// Paginated events API response.
#[derive(Serialize)]
pub struct EventsPage {
    pub events: Vec<GroupedEvent>,
    pub total_events: usize,
    pub total_markets: usize,
    pub page: usize,
    pub page_size: usize,
    pub has_more: bool,
    /// Available classes for the filter dropdown (populated on page 0 only).
    pub classes: Vec<FilterOption>,
}
