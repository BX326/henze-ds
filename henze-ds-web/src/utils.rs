//! Business logic utilities for data processing.

use henze_ds::{available_sports, HenzeFilter, HenzeInfo};
use std::collections::HashMap;

use crate::models::{BetsContext, FilterOption, GroupedEvent, MarketInfo, SportOption};

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
                event_time_utc: bet.event_time_utc.clone(),
                event_url: bet.event_url.clone(),
                is_live: bet.is_live,
                match_minute: bet.match_minute,
                sport_name: bet.sport_name.clone(),
                category_id: bet.category_id.clone(),
                category_name: bet.category_name.clone(),
                class_id: bet.class_id.clone(),
                class_name: bet.class_name.clone(),
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

/// Build filter options (categories or classes) from grouped events, sorted alphabetically.
pub fn build_filter_options(
    events: &[GroupedEvent],
    get_id: impl Fn(&GroupedEvent) -> &str,
    get_name: impl Fn(&GroupedEvent) -> &str,
) -> Vec<FilterOption> {
    let mut counts: HashMap<(String, String), usize> = HashMap::new();
    
    for event in events {
        let id = get_id(event).to_string();
        let name = get_name(event).to_string();
        *counts.entry((id, name)).or_insert(0) += 1;
    }
    
    let mut options: Vec<FilterOption> = counts
        .into_iter()
        .map(|((id, name), count)| FilterOption { id, name, count })
        .collect();
    
    // Sort alphabetically by name (ascending)
    options.sort_by(|a, b| a.name.cmp(&b.name));
    options
}

/// Build category (league/competition) filter options from events.
pub fn build_category_options(events: &[GroupedEvent]) -> Vec<FilterOption> {
    build_filter_options(events, |e| &e.category_id, |e| &e.category_name)
}

/// Build class (country/region) filter options from events.
pub fn build_class_options(events: &[GroupedEvent]) -> Vec<FilterOption> {
    build_filter_options(events, |e| &e.class_id, |e| &e.class_name)
}

/// Fetch bets and build the complete template context.
/// Note: Advanced filtering (time, league, live status) is handled client-side.
pub async fn fetch_bets_context(
    filter: HenzeFilter,
    selected_sport: String,
) -> BetsContext {
    let target = filter.target;
    let tolerance = filter.tolerance;
    let min_odds = filter.min_odds();
    let max_odds = filter.max_odds();
    let sports = build_sports_list(&selected_sport);

    match henze_ds::retrieve_henze_data_with_filter(filter).await {
        Ok(bets) => {
            let grouped_events = group_bets_by_event(&bets);
            let categories = build_category_options(&grouped_events);
            let classes = build_class_options(&grouped_events);
            
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
                categories,
                classes,
                // Default values for client-side filtering
                time_preset: "all".to_string(),
                from_time: String::new(),
                to_time: String::new(),
                live_only: false,
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
            categories: vec![],
            classes: vec![],
            time_preset: "all".to_string(),
            from_time: String::new(),
            to_time: String::new(),
            live_only: false,
        },
    }
}
