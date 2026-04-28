//! HTTP route handlers for the web application.

use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone, Utc};
use henze_ds::{HenzeFilter, HenzeInfo, DEFAULT_SPORT, DEFAULT_TARGET_ODDS, DEFAULT_TOLERANCE};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::{catch, get, post, uri};
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::env;

use crate::models::{EventsPage, ShellContext};
use crate::utils::{
    build_sports_list, fetch_bets_with_cache, fetch_events_page,
    prefetch_standard_windows, record_prefetch_time, get_last_prefetch_time,
};

/// Parse time preset into start/end DateTime range.
fn parse_time_preset(preset: &str) -> (Option<DateTime<Utc>>, Option<DateTime<Utc>>) {
    let now = Utc::now();
    let today_start = Utc.from_utc_datetime(
        &Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap(),
    );
    match preset {
        "next2h" => (Some(now), Some(now + Duration::hours(2))),
        "today" => (Some(today_start), Some(today_start + Duration::days(1))),
        "tomorrow" => {
            let tomorrow_start = today_start + Duration::days(1);
            (Some(tomorrow_start), Some(tomorrow_start + Duration::days(1)))
        }
        "week" => (Some(now), Some(now + Duration::days(7))),
        _ => (None, None),
    }
}

/// Parse ISO 8601 datetime string to DateTime<Utc>.
fn parse_datetime(s: &str) -> Option<DateTime<Utc>> {
    if s.is_empty() {
        return None;
    }
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M") {
        return Some(Utc.from_utc_datetime(&dt));
    }
    if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap()));
    }
    None
}

fn build_shell_context(target: f64, tolerance: f64, selected_sport: String) -> ShellContext {
    ShellContext {
        min_odds: target - tolerance,
        max_odds: target + tolerance,
        target,
        tolerance,
        sports: build_sports_list(&selected_sport),
        selected_sport,
    }
}

/// Main page — returns a thin shell with no event data.
/// Event data is loaded by JS via /api/events.
#[get("/?<target>&<tolerance>&<sport>")]
pub fn index(target: Option<f64>, tolerance: Option<f64>, sport: Option<String>) -> Template {
    let target = target.unwrap_or(DEFAULT_TARGET_ODDS);
    let tolerance = tolerance.unwrap_or(DEFAULT_TOLERANCE);
    let selected_sport = sport.unwrap_or_else(|| DEFAULT_SPORT.to_string());
    Template::render("index", build_shell_context(target, tolerance, selected_sport))
}

/// Paginated events API — returns grouped events as JSON.
/// Supports all filter params plus pagination (page, page_size) and class_id.
#[get("/api/events?<target>&<tolerance>&<sport>&<time_preset>&<from_time>&<to_time>&<live_only>&<page>&<page_size>&<class_id>")]
pub async fn api_events(
    target: Option<f64>,
    tolerance: Option<f64>,
    sport: Option<String>,
    time_preset: Option<String>,
    from_time: Option<String>,
    to_time: Option<String>,
    live_only: Option<bool>,
    page: Option<usize>,
    page_size: Option<usize>,
    class_id: Option<String>,
) -> Result<Json<EventsPage>, Status> {
    let sport_filter = sport.filter(|s| !s.is_empty());
    let preset = time_preset.unwrap_or_else(|| "all".to_string());
    let from = from_time.unwrap_or_default();
    let to = to_time.unwrap_or_default();
    let live = live_only.unwrap_or(false);
    let page = page.unwrap_or(0);
    let page_size = page_size.unwrap_or(40).clamp(1, 200);

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

    match fetch_events_page(filter, class_id, page, page_size).await {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::InternalServerError),
    }
}

/// Raw bets API — returns flat HenzeInfo JSON array (kept for external consumers).
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

    match fetch_bets_with_cache(filter).await {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub struct PrefetchAuth;

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub last_prefetch: Option<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for PrefetchAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let provided = request.headers().get_one("x-prefetch-token");
        let expected = env::var("PREFETCH_TOKEN").ok();

        match (provided, expected.as_deref()) {
            (Some(provided_token), Some(expected_token))
                if !expected_token.is_empty() && provided_token == expected_token =>
            {
                Outcome::Success(PrefetchAuth)
            }
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

#[post("/internal/prefetch")]
pub async fn internal_prefetch(_auth: PrefetchAuth) -> Status {
    match prefetch_standard_windows(true).await {
        Ok(()) => {
            record_prefetch_time();
            Status::NoContent
        }
        Err(err) => {
            eprintln!("Prefetch failed: {}", err);
            Status::InternalServerError
        }
    }
}

#[get("/internal/prefetch")]
pub fn internal_prefetch_status() -> Json<HealthStatus> {
    let last_prefetch = get_last_prefetch_time().map(|dt| dt.to_rfc3339());
    Json(HealthStatus {
        status: "ok".to_string(),
        last_prefetch,
    })
}

/// 404 handler — redirects to index.
#[catch(404)]
pub fn not_found() -> Redirect {
    Redirect::to(uri!(index(target = _, tolerance = _, sport = _)))
}
