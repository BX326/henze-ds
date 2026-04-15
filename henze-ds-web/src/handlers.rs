//! HTTP route handlers for the web application.

use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone, Utc};
use henze_ds::{HenzeFilter, HenzeInfo, DEFAULT_SPORT, DEFAULT_TARGET_ODDS, DEFAULT_TOLERANCE};
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::{catch, get, post, uri};
use rocket_dyn_templates::Template;

use crate::models::FilterParams;
use crate::utils::fetch_bets_context;

/// Parse time preset into start/end DateTime range (for API use)
fn parse_time_preset(preset: &str) -> (Option<DateTime<Utc>>, Option<DateTime<Utc>>) {
    let now = Utc::now();
    let today_start = Utc.from_utc_datetime(
        &Local::now()
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap(),
    );
    
    match preset {
        "next2h" => (Some(now), Some(now + Duration::hours(2))),
        "today" => (Some(today_start), Some(today_start + Duration::days(1))),
        "tomorrow" => {
            let tomorrow_start = today_start + Duration::days(1);
            (Some(tomorrow_start), Some(tomorrow_start + Duration::days(1)))
        }
        "week" => (Some(now), Some(now + Duration::days(7))),
        _ => (None, None), // "all" or invalid
    }
}

/// Parse ISO 8601 datetime string to DateTime<Utc> (for API use)
fn parse_datetime(s: &str) -> Option<DateTime<Utc>> {
    if s.is_empty() {
        return None;
    }
    // Try parsing as RFC3339 first
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    // Try parsing datetime-local format (YYYY-MM-DDTHH:MM)
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M") {
        return Some(Utc.from_utc_datetime(&dt));
    }
    // Try parsing date-only format
    if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap()));
    }
    None
}

/// Main page - shows the form and initial bets with default filter.
#[get("/")]
pub async fn index() -> Template {
    let filter = HenzeFilter::with_sport(
        DEFAULT_TARGET_ODDS,
        DEFAULT_TOLERANCE,
        Some(DEFAULT_SPORT.to_string()),
    );
    let ctx = fetch_bets_context(filter, DEFAULT_SPORT.to_string()).await;
    Template::render("index", ctx)
}

/// Form submission - fetches bets with custom filter.
/// Note: Advanced filtering (time, league, live status) is handled client-side.
#[post("/", data = "<params>")]
pub async fn filter_bets(params: Form<FilterParams>) -> Template {
    let target = params.target.unwrap_or(DEFAULT_TARGET_ODDS);
    let tolerance = params.tolerance.unwrap_or(DEFAULT_TOLERANCE);
    let sport = params
        .sport
        .clone()
        .unwrap_or_else(|| DEFAULT_SPORT.to_string());
    
    let mut filter = HenzeFilter::with_sport(target, tolerance, Some(sport.clone()));
    
    // Handle empty sport (all sports)
    if sport.is_empty() {
        filter.sport_tag_id = None;
    }
    
    let mut ctx = fetch_bets_context(filter, sport.clone()).await;
    ctx.selected_sport = sport;
    Template::render("index", ctx)
}

/// API endpoint - returns JSON data.
/// Supports optional time filtering for programmatic access.
#[get("/api/bets?<target>&<tolerance>&<sport>&<time_preset>&<from_time>&<to_time>&<live_only>")]
pub async fn api_bets(
    target: Option<f64>,
    tolerance: Option<f64>,
    sport: Option<String>,
    time_preset: Option<String>,
    from_time: Option<String>,
    to_time: Option<String>,
    live_only: Option<bool>,
) -> Result<Json<Vec<HenzeInfo>>, Status> {
    let sport_filter = sport.filter(|s| !s.is_empty());
    let preset = time_preset.unwrap_or_else(|| "all".to_string());
    let from = from_time.unwrap_or_default();
    let to = to_time.unwrap_or_default();
    let live = live_only.unwrap_or(false);
    
    // Determine time range (for API filtering)
    let (start_from, start_to) = if preset == "custom" {
        (parse_datetime(&from), parse_datetime(&to))
    } else {
        parse_time_preset(&preset)
    };
    
    let filter = HenzeFilter::with_sport(
        target.unwrap_or(DEFAULT_TARGET_ODDS),
        tolerance.unwrap_or(DEFAULT_TOLERANCE),
        sport_filter,
    )
    .with_time_range(start_from, start_to)
    .with_live_only(live);

    match henze_ds::retrieve_henze_data_with_filter(filter).await {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::InternalServerError),
    }
}

/// 404 handler - redirects to index.
#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to(uri!(index))
}
