//! Business logic utilities for data processing.

use henze_ds::{available_sports, HenzeFilter, HenzeInfo};
use std::collections::HashMap;

use crate::models::{BetsContext, GroupedEvent, MarketInfo, SportOption};

/// Group bets by event, sorting live events first.
pub fn group_bets_by_event(bets: &[HenzeInfo]) -> Vec<GroupedEvent> {
    let mut event_map: HashMap<String, GroupedEvent> = HashMap::new();

    for bet in bets {
        let entry = event_map
            .entry(bet.event_id.clone())
            .or_insert_with(|| GroupedEvent {
                event_id: bet.event_id.clone(),
                event_name: bet.event_name.clone(),
                event_time: bet.event_time.clone(),
                event_url: bet.event_url.clone(),
                is_live: bet.is_live,
                match_minute: bet.match_minute,
                sport_name: bet.sport_name.clone(),
                markets: Vec::new(),
            });

        entry.markets.push(MarketInfo {
            market_name: bet.market_name.clone(),
            outcome: bet.outcome.clone(),
            decimal: bet.decimal,
        });
    }

    let mut events: Vec<GroupedEvent> = event_map.into_values().collect();
    // Sort: live events first, then by time
    events.sort_by(|a, b| match (a.is_live, b.is_live) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.event_time.cmp(&b.event_time),
    });
    events
}

/// Build the sports dropdown list with an "All" option.
pub fn build_sports_list(selected: &str) -> Vec<SportOption> {
    let mut list: Vec<SportOption> = Vec::new();
    // Add an 'All' option with empty id
    list.push(SportOption {
        id: "".to_string(),
        name: "All".to_string(),
        selected: selected.is_empty(),
    });
    available_sports().into_iter().for_each(|(id, name)| {
        list.push(SportOption {
            id: id.to_string(),
            name: name.to_string(),
            selected: id == selected,
        })
    });
    list
}

/// Fetch bets and build the complete template context.
pub async fn fetch_bets_context(filter: HenzeFilter, selected_sport: String) -> BetsContext {
    let target = filter.target;
    let tolerance = filter.tolerance;
    let min_odds = filter.min_odds();
    let max_odds = filter.max_odds();
    let sports = build_sports_list(&selected_sport);

    match henze_ds::retrieve_henze_data_with_filter(filter).await {
        Ok(bets) => {
            let grouped_events = group_bets_by_event(&bets);
            BetsContext {
                count: bets.len(),
                event_count: grouped_events.len(),
                grouped_events,
                bets,
                target,
                tolerance,
                min_odds,
                max_odds,
                error: None,
                sports,
                selected_sport,
            }
        }
        Err(e) => BetsContext {
            bets: vec![],
            grouped_events: vec![],
            target,
            tolerance,
            min_odds,
            max_odds,
            count: 0,
            event_count: 0,
            error: Some(format!("Failed to fetch data: {}", e)),
            sports,
            selected_sport,
        },
    }
}
