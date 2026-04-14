//! Henze CLI - Command-line tool for fetching and displaying bets.
//!
//! # Usage
//! ```sh
//! henze-ds-cli [target_odds] [tolerance]
//! ```
//!
//! Default: target=1.1, tolerance=0.04

mod cli;
mod output;

use std::error::Error;

use cli::{create_filter, parse_args};
use output::{display_filter, display_results};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect();
    let cli_args = parse_args(args);
    let filter = create_filter(&cli_args);

    display_filter(&filter);

    let results = henze_ds::retrieve_henze_data_with_filter(filter).await?;

    display_results(&results);

    Ok(())
}