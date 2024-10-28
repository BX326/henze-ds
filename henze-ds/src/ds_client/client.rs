use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

use super::schema::TimeBandEventList;

const BASE_URL: &str = "https://content.sb.danskespil.dk/content-service/api/v1/q";
const ENDPOINT: &str = "time-band-event-list?maxMarkets=9999&marketSortsIncluded=--,H1,H2,HH,HL,MH,MR,WH&marketGroupTypesIncluded=CUSTOM_GROUP,MATCH_RESULT,MONEYLINE,STATIC_SPREAD,STATIC_TOTAL&allowedEventSorts=MTCH&includeChildMarkets=true&prioritisePrimaryMarkets=true&includeCommentary=true&includeIncidents=true&includeMedia=true&drilldownTagIds=12&excludeDrilldownTagIds=20769,22796,22797,22800&maxTotalItems=120&maxEventsPerCompetition=99&maxCompetitionsPerSportPerBand=99&maxEventsForNextToGo=99&startTimeOffsetForNextToGo=600";

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
        let url = format!("{}/{}", self.base_url, ENDPOINT);
        let response = self.client.get(&url).send().await?.json::<TimeBandEventList>().await?;
        Ok(response)
    }
}