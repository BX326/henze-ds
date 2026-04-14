//! HTTP route handlers for the web application.

use henze_ds::{HenzeFilter, HenzeInfo, DEFAULT_SPORT, DEFAULT_TARGET_ODDS, DEFAULT_TOLERANCE};
use rocket::form::Form;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::{catch, get, post, uri};
use rocket_dyn_templates::Template;

use crate::models::FilterParams;
use crate::utils::fetch_bets_context;

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
#[post("/", data = "<params>")]
pub async fn filter_bets(params: Form<FilterParams>) -> Template {
    let target = params.target.unwrap_or(DEFAULT_TARGET_ODDS);
    let tolerance = params.tolerance.unwrap_or(DEFAULT_TOLERANCE);
    let sport = params
        .sport
        .clone()
        .unwrap_or_else(|| DEFAULT_SPORT.to_string());
    let filter = HenzeFilter::with_sport(target, tolerance, Some(sport.clone()));
    let mut ctx = fetch_bets_context(filter, sport.clone()).await;
    ctx.selected_sport = sport;
    Template::render("index", ctx)
}

/// API endpoint - returns JSON data.
#[get("/api/bets?<target>&<tolerance>&<sport>")]
pub async fn api_bets(
    target: Option<f64>,
    tolerance: Option<f64>,
    sport: Option<String>,
) -> Result<Json<Vec<HenzeInfo>>, Status> {
    let sport_filter = sport.filter(|s| !s.is_empty());
    let filter = HenzeFilter::with_sport(
        target.unwrap_or(DEFAULT_TARGET_ODDS),
        tolerance.unwrap_or(DEFAULT_TOLERANCE),
        sport_filter,
    );

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
