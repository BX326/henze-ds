use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::schema::TimeBandEventList;

const BASE_URL: &str = "https://content.sb.danskespil.dk/content-service/api/v1/q";
const BASE_ENDPOINT: &str = "time-band-event-list?maxMarkets=9999&marketSortsIncluded=--,H1,H2,HH,HL,MH,MR,WH&marketGroupTypesIncluded=CUSTOM_GROUP,MATCH_RESULT,MONEYLINE,STATIC_SPREAD,STATIC_TOTAL&allowedEventSorts=MTCH&includeChildMarkets=true&prioritisePrimaryMarkets=true&includeCommentary=true&includeIncidents=true&includeMedia=true&excludeDrilldownTagIds=20769,22796,22797,22800&maxTotalItems=120&maxEventsPerCompetition=99&maxCompetitionsPerSportPerBand=99&maxEventsForNextToGo=99&startTimeOffsetForNextToGo=600";

/// Known sport drilldown tag IDs
pub const SPORT_FOOTBALL: &str = "12";
pub const SPORT_TENNIS: &str = "854";
pub const SPORT_BASKETBALL: &str = "465";
pub const SPORT_ICE_HOCKEY: &str = "786";
pub const SPORT_HANDBALL: &str = "664";
pub const SPORT_VOLLEYBALL: &str = "404";
pub const SPORT_ESPORTS: &str = "977";
pub const SPORT_TABLE_TENNIS: &str = "820";
pub const SPORT_DARTS: &str = "783";
pub const SPORT_AMERICAN_FOOTBALL: &str = "720";

/// Available sports for filtering
pub fn available_sports() -> Vec<(&'static str, &'static str)> {
    vec![
        (SPORT_FOOTBALL, "Football"),
        (SPORT_TENNIS, "Tennis"),
        (SPORT_BASKETBALL, "Basketball"),
        (SPORT_ICE_HOCKEY, "Ice Hockey"),
        (SPORT_HANDBALL, "Handball"),
        (SPORT_VOLLEYBALL, "Volleyball"),
        (SPORT_ESPORTS, "eSports"),
        (SPORT_TABLE_TENNIS, "Table Tennis"),
        (SPORT_DARTS, "Darts"),
        (SPORT_AMERICAN_FOOTBALL, "American Football"),
    ]
}

/// Default sport tag ID (Football)
pub const DEFAULT_SPORT: &str = SPORT_FOOTBALL;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    event: String,
    outcome: String,
    odds: f32,
}

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        ApiClient {
            client: Client::new(),
            base_url: BASE_URL.to_string(),
        }
    }

    pub async fn get_data(&self) -> Result<TimeBandEventList, Box<dyn Error>> {
        self.get_data_with_sport(None).await
    }

    pub async fn get_data_with_sport(&self, sport_tag_id: Option<&str>) -> Result<TimeBandEventList, Box<dyn Error>> {
        // If a non-empty sport_tag_id is provided, include drilldownTagIds param.
        // If None or empty, do not include drilldownTagIds so the API returns all sports.
        let url = if let Some(tag) = sport_tag_id.filter(|s| !s.is_empty()) {
            format!("{}/{}&drilldownTagIds={}", self.base_url, BASE_ENDPOINT, tag)
        } else {
            format!("{}/{}", self.base_url, BASE_ENDPOINT)
        };
        let response_text = self.client.get(&url).send().await?.text().await?;
        match serde_json::from_str::<TimeBandEventList>(&response_text) {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                eprintln!("JSON parse error: {} at line {} col {}", e, e.line(), e.column());
                Err(Box::new(e))
            }
        }
    }
}