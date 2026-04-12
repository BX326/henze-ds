use serde::Serialize;

pub mod ds_client;

/// Default target odds for Henze bets
pub const DEFAULT_TARGET_ODDS: f64 = 1.1;

/// Default tolerance around the target odds
pub const DEFAULT_TOLERANCE: f64 = 0.04;

#[derive(Debug, Serialize)]
pub struct HenzeInfo {
    pub event_id: String,
    pub event_name: String,
    pub event_time: String,
    pub market_name: String,
    pub outcome: String,
    pub decimal: f64,
    pub event_url: String,
}

/// Parameters for filtering Henze bets
pub struct HenzeFilter {
    /// Target odds (default: 1.1)
    pub target: f64,
    /// Tolerance around target (default: 0.04, meaning 1.06 to 1.14)
    pub tolerance: f64,
}

impl Default for HenzeFilter {
    fn default() -> Self {
        Self {
            target: DEFAULT_TARGET_ODDS,
            tolerance: DEFAULT_TOLERANCE,
        }
    }
}

impl HenzeFilter {
    pub fn new(target: f64, tolerance: f64) -> Self {
        Self { target, tolerance }
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

pub async fn retrieve_henze_data_with_filter(
    filter: HenzeFilter,
) -> Result<Vec<HenzeInfo>, Box<dyn std::error::Error>> {
    let data = ds_client::client::ApiClient::new().get_data().await?;

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
                let event_time = event.start_time;
                let event_url = format!("https://danskespil.dk/oddset/in-play/event/{}", event_id);
                event.markets.iter().flat_map(move |market| {
                    let market_name = &market.name;
                    let event_url = event_url.clone();
                    market.outcomes.iter().flat_map(move |outcome| {
                        let outcome_name = &outcome.name;
                        let event_url = event_url.clone();
                        outcome.prices.iter().filter_map(move |price| {
                            (price.decimal >= min_odds && price.decimal <= max_odds).then(|| HenzeInfo {
                                event_id: event_id.clone(),
                                event_name: event_name.clone(),
                                event_time: event_time.clone().to_string(),
                                market_name: market_name.clone().to_string(),
                                outcome: outcome_name.clone(),
                                decimal: price.decimal,
                                event_url: event_url.clone(),
                            })
                        })
                    })
                })
            })
        })
        .collect();

    Ok(collected_info)
}