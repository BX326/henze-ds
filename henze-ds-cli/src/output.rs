//! Output formatting and display utilities.

use henze_ds::{HenzeFilter, HenzeInfo};

/// Display the filter parameters being used.
pub fn display_filter(filter: &HenzeFilter) {
    println!(
        "Filtering for odds {:.2} ± {:.2} (range: {:.2} - {:.2})",
        filter.target,
        filter.tolerance,
        filter.min_odds(),
        filter.max_odds()
    );
}

/// Display the fetched bet results.
pub fn display_results(results: &[HenzeInfo]) {
    println!("Found {} matching bets:\n", results.len());
    for info in results {
        println!("{:?}", info);
    }
}
