use henze_ds::{available_sports, HenzeFilter, HenzeInfo, DEFAULT_SPORT, DEFAULT_TARGET_ODDS, DEFAULT_TOLERANCE};

use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::{self, catch, catchers, get, launch, post, routes, uri, FromForm};
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::collections::HashMap;
use std::env;

#[derive(FromForm)]
struct FilterParams {
    target: Option<f64>,
    tolerance: Option<f64>,
    sport: Option<String>,
}

#[derive(Serialize, Clone)]
struct MarketInfo {
    market_name: String,
    outcome: String,
    decimal: f64,
}

#[derive(Serialize, Clone)]
struct GroupedEvent {
    event_id: String,
    event_name: String,
    event_time: String,
    event_url: String,
    is_live: bool,
    match_minute: Option<i32>,
    sport_name: String,
    markets: Vec<MarketInfo>,
}

#[derive(Serialize)]
struct SportOption {
    id: String,
    name: String,
    selected: bool,
}

#[derive(Serialize)]
struct BetsContext {
    bets: Vec<HenzeInfo>,
    grouped_events: Vec<GroupedEvent>,
    target: f64,
    tolerance: f64,
    min_odds: f64,
    max_odds: f64,
    count: usize,
    event_count: usize,
    error: Option<String>,
    sports: Vec<SportOption>,
    selected_sport: String,
}

/// Group bets by event
fn group_bets_by_event(bets: &[HenzeInfo]) -> Vec<GroupedEvent> {
    let mut event_map: HashMap<String, GroupedEvent> = HashMap::new();
    
    for bet in bets {
        let entry = event_map.entry(bet.event_id.clone()).or_insert_with(|| GroupedEvent {
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
    events.sort_by(|a, b| {
        match (a.is_live, b.is_live) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.event_time.cmp(&b.event_time),
        }
    });
    events
}

/// Main page - shows the form and initial bets with default filter
#[get("/")]
async fn index() -> Template {
    let filter = HenzeFilter::with_sport(DEFAULT_TARGET_ODDS, DEFAULT_TOLERANCE, Some(DEFAULT_SPORT.to_string()));
    let ctx = fetch_bets_context(filter, DEFAULT_SPORT.to_string()).await;
    Template::render("index", ctx)
}

/// Form submission - fetches bets with custom filter
#[post("/", data = "<params>")]
async fn filter_bets(params: Form<FilterParams>) -> Template {
    let target = params.target.unwrap_or(DEFAULT_TARGET_ODDS);
    let tolerance = params.tolerance.unwrap_or(DEFAULT_TOLERANCE);
    let sport = params.sport.clone().unwrap_or_else(|| DEFAULT_SPORT.to_string());
    let filter = HenzeFilter::with_sport(target, tolerance, Some(sport.clone()));
    let mut ctx = fetch_bets_context(filter, sport.clone()).await;
    ctx.selected_sport = sport;
    Template::render("index", ctx)
}

/// API endpoint - returns JSON data
#[get("/api/bets?<target>&<tolerance>&<sport>")]
async fn api_bets(target: Option<f64>, tolerance: Option<f64>, sport: Option<String>) -> Result<Json<Vec<HenzeInfo>>, Status> {
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

fn build_sports_list(selected: &str) -> Vec<SportOption> {
    let mut list: Vec<SportOption> = Vec::new();
    // Add an 'All' option with empty id
    list.push(SportOption { id: "".to_string(), name: "All".to_string(), selected: selected.is_empty() });
    available_sports()
        .into_iter()
        .for_each(|(id, name)| list.push(SportOption { id: id.to_string(), name: name.to_string(), selected: id == selected }));
    list
}

async fn fetch_bets_context(filter: HenzeFilter, selected_sport: String) -> BetsContext {
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
        },
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

#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to(uri!(index))
}

#[launch]
fn rocket() -> _ {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16 integer");

    // Get static files path - use env var for Docker, fallback to CARGO_MANIFEST_DIR for dev
    let static_path = env::var("ROCKET_STATIC_DIR")
        .unwrap_or_else(|_| format!("{}/static", env!("CARGO_MANIFEST_DIR")));

    rocket::build()
        .mount("/", routes![index, filter_bets, api_bets])
        .mount("/static", FileServer::from(static_path))
        .attach(Template::fairing())
        .register("/", catchers![not_found])
        .configure(rocket::Config {
            port,
            address: "0.0.0.0".parse().unwrap(),
            ..rocket::Config::default()
        })
}
