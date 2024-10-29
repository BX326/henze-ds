use serde::Serialize;

pub mod ds_client;

#[derive(Debug, Serialize)]
pub struct HenzeInfo {
    event_name: String,
    event_time: String,
    market_name: String,
    outcome: String,
    decimal: f64,
}

pub async fn retrieve_henze_data() -> Result<Vec<HenzeInfo>, Box<dyn std::error::Error>> {
    let data = ds_client::client::ApiClient::new().get_data().await?;

    let collected_info: Vec<HenzeInfo> = data
        .data
        .time_band_events
        .iter()
        .flat_map(|time_band_event| {
            time_band_event.events.iter().flat_map(move |event| {
                let event_name = &event.name;
                let event_time = event.start_time;
                event.markets.iter().flat_map(move |market| {
                    let market_name = &market.name;
                    market.outcomes.iter().flat_map(move |outcome| {
                        let outcome_name = &outcome.name;
                        outcome.prices.iter().filter_map(move |price| {
                            (1.08 <= price.decimal && price.decimal <= 1.12).then(|| HenzeInfo {
                                event_name: event_name.clone(),
                                event_time: event_time.clone().to_string(),
                                market_name: market_name.clone().to_string(),
                                outcome: outcome_name.clone(),
                                decimal: price.decimal,
                            })
                        })
                    })
                })
            })
        })
        .collect();

    Ok(collected_info)
}