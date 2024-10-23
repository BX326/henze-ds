mod ds_client;

use std::error::Error;
use tokio;
use ds_client::client::ApiClient;

#[derive(Debug)]
struct HenzeInfo {
    match_name: String,
    market: String,
    outcome: String,
    decimal: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_client = ApiClient::new();
    let response = api_client.get_data().await?;
    println!("{:?}", response.data.time_band_events[0].events[0].markets[0].outcomes[0].prices[0].decimal);

    Ok(())
}
