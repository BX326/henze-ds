//! Business logic utilities for data processing.

use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use chrono_tz::Europe::Copenhagen;
use henze_ds::{available_sports, HenzeFilter, HenzeInfo};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::sync::{Mutex, OnceLock};

use crate::cache::ResponseCache;
use crate::models::{BetsContext, FilterOption, GroupedEvent, MarketInfo, SportOption};

static RESPONSE_CACHE: OnceLock<Option<ResponseCache>> = OnceLock::new();
static LAST_PREFETCH_TIME: Mutex<Option<DateTime<Utc>>> = Mutex::new(None);

fn response_cache() -> Option<&'static ResponseCache> {
    RESPONSE_CACHE
        .get_or_init(|| {
            let path = env::var("CACHE_DB_PATH")
                .unwrap_or_else(|_| "/tmp/henze/events_cache.sqlite".to_string());
            match ResponseCache::new(&path) {
                Ok(cache) => Some(cache),
                Err(err) => {
                    eprintln!("Failed to initialize cache at {}: {}", path, err);
                    None
                }
            }
        })
        .as_ref()
}

fn cache_key_for_filter(filter: &HenzeFilter) -> String {
    let from = filter
        .start_time_from
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_else(|| "none".to_string());
    let to = filter
        .start_time_to
        .map(|dt| dt.to_rfc3339())
        .unwrap_or_else(|| "none".to_string());
    let sport = filter
        .sport_tag_id
        .as_deref()
        .unwrap_or("all");

    format!(
        "bets:v1:sport={}:target={:.3}:tol={:.3}:from={}:to={}:live={}:include_started={}",
        sport,
        filter.target,
        filter.tolerance,
        from,
        to,
        filter.live_only,
        filter.include_started
    )
}

pub async fn prefetch_standard_windows(force_refresh: bool) -> Result<(), Box<dyn Error>> {
    for day_offset in [0_i64, 1, 2] {
        let Some((from, to)) = copenhagen_day_bounds(Utc::now(), day_offset) else {
            continue;
        };

        // Prefetch only non-started events for today/tomorrow/day-after windows.
        let filter = HenzeFilter::default()
            .with_time_range(Some(from), Some(to))
            .with_include_started(false);

        let _ = fetch_bets_with_cache_control(filter, force_refresh).await?;
    }

    Ok(())
}

pub fn record_prefetch_time() {
    if let Ok(mut lock) = LAST_PREFETCH_TIME.lock() {
        *lock = Some(Utc::now());
    }
}

pub fn get_last_prefetch_time() -> Option<DateTime<Utc>> {
    LAST_PREFETCH_TIME.lock().ok().and_then(|lock| *lock)
}

fn copenhagen_day_bounds(now_utc: DateTime<Utc>, days_offset: i64) -> Option<(DateTime<Utc>, DateTime<Utc>)> {
    let local_now = now_utc.with_timezone(&Copenhagen);
    let date = local_now.date_naive() + Duration::days(days_offset);
    let local_start = Copenhagen
        .with_ymd_and_hms(date.year(), date.month(), date.day(), 0, 0, 0)
        .single()?;
    let local_end = local_start + Duration::days(1);
    Some((local_start.with_timezone(&Utc), local_end.with_timezone(&Utc)))
}

fn time_range_matches(range: (DateTime<Utc>, DateTime<Utc>), from: DateTime<Utc>, to: DateTime<Utc>) -> bool {
    range.0.timestamp() == from.timestamp() && range.1.timestamp() == to.timestamp()
}

fn ttl_seconds_for_filter(filter: &HenzeFilter) -> i64 {
    let now = Utc::now();

    match (filter.start_time_from, filter.start_time_to) {
        (Some(from), Some(to)) => {
            let today = copenhagen_day_bounds(now, 0);
            let tomorrow = copenhagen_day_bounds(now, 1);
            let day_after = copenhagen_day_bounds(now, 2);

            if today.map(|r| time_range_matches(r, from, to)).unwrap_or(false) {
                return 6 * 3600;
            }
            if tomorrow.map(|r| time_range_matches(r, from, to)).unwrap_or(false)
                || day_after.map(|r| time_range_matches(r, from, to)).unwrap_or(false)
            {
                return 24 * 3600;
            }

            3600
        }
        _ => 6 * 3600,
    }
}

pub async fn fetch_bets_with_cache_control(
    filter: HenzeFilter,
    force_refresh: bool,
) -> Result<Vec<HenzeInfo>, Box<dyn Error>> {
    if filter.live_only {
        // Live requests should always be fetched fresh.
        return henze_ds::retrieve_henze_data_with_filter(filter).await;
    }

    let ttl = ttl_seconds_for_filter(&filter);
    let key = cache_key_for_filter(&filter);
    if !force_refresh {
        if let Some(cache) = response_cache() {
            if let Some(cached) = cache.get::<Vec<HenzeInfo>>(&key)? {
                return Ok(cached);
            }
        }
    }

    let fresh = henze_ds::retrieve_henze_data_with_filter(filter).await?;

    if let Some(cache) = response_cache() {
        if let Err(err) = cache.set(&key, &fresh, ttl) {
            eprintln!("Failed to write cache key {}: {}", key, err);
        }
        if let Err(err) = cache.purge_expired() {
            eprintln!("Failed to purge expired cache rows: {}", err);
        }
    }

    Ok(fresh)
}

pub async fn fetch_bets_with_cache(
    filter: HenzeFilter,
) -> Result<Vec<HenzeInfo>, Box<dyn Error>> {
    fetch_bets_with_cache_control(filter, false).await
}

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

    match fetch_bets_with_cache(filter).await {
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
