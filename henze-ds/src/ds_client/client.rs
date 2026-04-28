use reqwest::Client;
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;

use super::schema::TimeBandEventList;

const BASE_URL: &str = "https://content.sb.danskespil.dk/content-service/api/v1/q";
const BASE_ENDPOINT: &str = "time-band-event-list?maxMarkets=9999&marketSortsIncluded=--,H1,H2,HH,HL,MH,MR,WH&marketGroupTypesIncluded=CUSTOM_GROUP,MATCH_RESULT,MONEYLINE,STATIC_SPREAD,STATIC_TOTAL&allowedEventSorts=MTCH&includeChildMarkets=true&prioritisePrimaryMarkets=true&includeCommentary=true&includeIncidents=true&includeMedia=true&excludeDrilldownTagIds=20769,22796,22797,22800&maxTotalItems=120&maxEventsPerCompetition=99&maxCompetitionsPerSportPerBand=99&maxEventsForNextToGo=99&startTimeOffsetForNextToGo=600";

const EVENT_LIST_ENDPOINT: &str = "event-list";
const EVENT_LIST_EXCLUDED_DRILLDOWN_IDS: &str = "11341,20769,22796,22797";

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

#[derive(Debug, Clone)]
pub struct EventListQuery {
    pub start_time_from: DateTime<Utc>,
    pub start_time_to: DateTime<Utc>,
    pub started: bool,
    pub sport_tag_id: Option<String>,
    pub max_events: u32,
    pub max_markets: u32,
    pub lang: String,
    pub channel: String,
}

impl EventListQuery {
    pub fn new(start_time_from: DateTime<Utc>, start_time_to: DateTime<Utc>, started: bool) -> Self {
        Self {
            start_time_from,
            start_time_to,
            started,
            sport_tag_id: None,
            max_events: 99_999,
            max_markets: 99_999,
            lang: "da-DK".to_string(),
            channel: "I".to_string(),
        }
    }

    pub fn with_sport(mut self, sport_tag_id: Option<String>) -> Self {
        self.sport_tag_id = sport_tag_id.filter(|s| !s.is_empty());
        self
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventListResponse {
    #[serde(default)]
    pub data: EventListData,
}

fn deserialize_events<'de, D>(deserializer: D) -> Result<Vec<EventListEvent>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize;
    match Option::<Vec<EventListEvent>>::deserialize(deserializer) {
        Ok(Some(events)) => Ok(events),
        Ok(None) => Ok(vec![]),
        Err(_) => Ok(vec![]),
    }
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventListData {
    #[serde(default, deserialize_with = "deserialize_events")]
    pub events: Vec<EventListEvent>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventClass {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventCategory {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventClock {
    pub offset: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventCommentary {
    pub clock: Option<EventClock>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventPrice {
    pub decimal: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventOutcome {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub resulted: bool,
    #[serde(default)]
    pub status: String,
    #[serde(default, deserialize_with = "deserialize_prices")]
    pub prices: Vec<EventPrice>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventMarket {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, rename = "type")]
    pub market_type: String,
    #[serde(default, rename = "subType")]
    pub market_sub_type: String,
    #[serde(default, deserialize_with = "deserialize_outcomes")]
    pub outcomes: Vec<EventOutcome>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventListEvent {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, deserialize_with = "deserialize_start_time")]
    pub start_time: DateTime<Utc>,
    #[serde(default)]
    pub live_now: bool,
    #[serde(default)]
    pub sport_id: String,
    #[serde(default)]
    pub category: EventCategory,
    #[serde(default, rename = "class")]
    pub class_field: EventClass,
    #[serde(default)]
    pub commentary: Option<EventCommentary>,
    #[serde(default, deserialize_with = "deserialize_markets")]
    pub markets: Vec<EventMarket>,
}

fn deserialize_start_time<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize;
    let opt = Option::<DateTime<Utc>>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_else(|| Utc::now()))
}

fn deserialize_markets<'de, D>(deserializer: D) -> Result<Vec<EventMarket>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize;
    match Option::<Vec<EventMarket>>::deserialize(deserializer) {
        Ok(Some(markets)) => Ok(markets),
        Ok(None) => Ok(vec![]),
        Err(e) => {
            eprintln!("Warning: Failed to deserialize markets: {}", e);
            Ok(vec![])
        }
    }
}

fn deserialize_outcomes<'de, D>(deserializer: D) -> Result<Vec<EventOutcome>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize;
    match Option::<Vec<EventOutcome>>::deserialize(deserializer) {
        Ok(Some(outcomes)) => Ok(outcomes),
        Ok(None) => Ok(vec![]),
        Err(e) => {
            eprintln!("Warning: Failed to deserialize outcomes: {}", e);
            Ok(vec![])
        }
    }
}

fn deserialize_prices<'de, D>(deserializer: D) -> Result<Vec<EventPrice>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Deserialize;
    match Option::<Vec<EventPrice>>::deserialize(deserializer) {
        Ok(Some(prices)) => Ok(prices),
        Ok(None) => Ok(vec![]),
        Err(e) => {
            eprintln!("Warning: Failed to deserialize prices: {}", e);
            Ok(vec![])
        }
    }
}

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(20))
            .build()
            .expect("failed to build HTTP client");

        ApiClient {
            client,
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

    pub async fn get_event_list(&self, query: &EventListQuery) -> Result<EventListResponse, Box<dyn Error>> {
        let mut url = reqwest::Url::parse(&format!("{}/{}", self.base_url, EVENT_LIST_ENDPOINT))?;

        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair(
                "startTimeFrom",
                &query
                    .start_time_from
                    .to_rfc3339_opts(SecondsFormat::Secs, true),
            );
            pairs.append_pair(
                "startTimeTo",
                &query
                    .start_time_to
                    .to_rfc3339_opts(SecondsFormat::Secs, true),
            );
            pairs.append_pair("started", if query.started { "true" } else { "false" });
            pairs.append_pair("maxEvents", &query.max_events.to_string());
            pairs.append_pair("orderEventsBy", "startTime");
            pairs.append_pair("maxMarkets", &query.max_markets.to_string());
            pairs.append_pair("orderMarketsBy", "displayOrder");
            pairs.append_pair("eventSortsIncluded", "MTCH");
            pairs.append_pair("includeChildMarkets", "true");
            pairs.append_pair("prioritisePrimaryMarkets", "true");
            pairs.append_pair("includeMedia", "true");
            pairs.append_pair("excludeEventsWithNoMarkets", "true");
            pairs.append_pair("excludeDrilldownTagIds", EVENT_LIST_EXCLUDED_DRILLDOWN_IDS);
            pairs.append_pair("lang", &query.lang);
            pairs.append_pair("channel", &query.channel);

            if let Some(tag) = query.sport_tag_id.as_deref() {
                pairs.append_pair("drilldownTagIds", tag);
            }
        }

        let json_value = self.client.get(url.clone()).send().await?.json::<serde_json::Value>().await?;

        // Manually handle data.events null/malformed payloads gracefully.
        let mut response = EventListResponse::default();

        if let Some(data_obj) = json_value.get("data").and_then(|v| v.as_object()) {
            let mut data = EventListData::default();

            if let Some(events_val) = data_obj.get("events") {
                if !events_val.is_null() {
                    match serde_json::from_value::<Vec<EventListEvent>>(events_val.clone()) {
                        Ok(events) => {
                            data.events = events;
                        }
                        Err(e) => {
                            eprintln!("Failed to deserialize events field: {}", e);
                            data.events = vec![];
                        }
                    }
                } else {
                    data.events = vec![];
                }
            }
            response.data = data;
        }

        Ok(response)
    }
}