use henze_ds::{HenzeFilter, HenzeInfo, DEFAULT_TARGET_ODDS, DEFAULT_TOLERANCE};

use rocket::form::Form;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::{self, catch, catchers, get, launch, post, routes, uri, FromForm};
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::env;

#[derive(FromForm)]
struct FilterParams {
    target: Option<f64>,
    tolerance: Option<f64>,
}

#[derive(Serialize)]
struct BetsContext {
    bets: Vec<HenzeInfo>,
    target: f64,
    tolerance: f64,
    min_odds: f64,
    max_odds: f64,
    count: usize,
    error: Option<String>,
}

/// Main page - shows the form and initial bets with default filter
#[get("/")]
async fn index() -> Template {
    let filter = HenzeFilter::default();
    let ctx = fetch_bets_context(filter).await;
    Template::render("index", ctx)
}

/// Form submission - fetches bets with custom filter
#[post("/", data = "<params>")]
async fn filter_bets(params: Form<FilterParams>) -> Template {
    let target = params.target.unwrap_or(DEFAULT_TARGET_ODDS);
    let tolerance = params.tolerance.unwrap_or(DEFAULT_TOLERANCE);
    let filter = HenzeFilter::new(target, tolerance);
    let ctx = fetch_bets_context(filter).await;
    Template::render("index", ctx)
}

/// API endpoint - returns JSON data
#[get("/api/bets?<target>&<tolerance>")]
async fn api_bets(target: Option<f64>, tolerance: Option<f64>) -> Result<Json<Vec<HenzeInfo>>, Status> {
    let filter = HenzeFilter::new(
        target.unwrap_or(DEFAULT_TARGET_ODDS),
        tolerance.unwrap_or(DEFAULT_TOLERANCE),
    );

    match henze_ds::retrieve_henze_data_with_filter(filter).await {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::InternalServerError),
    }
}

async fn fetch_bets_context(filter: HenzeFilter) -> BetsContext {
    let target = filter.target;
    let tolerance = filter.tolerance;
    let min_odds = filter.min_odds();
    let max_odds = filter.max_odds();

    match henze_ds::retrieve_henze_data_with_filter(filter).await {
        Ok(bets) => BetsContext {
            count: bets.len(),
            bets,
            target,
            tolerance,
            min_odds,
            max_odds,
            error: None,
        },
        Err(e) => BetsContext {
            bets: vec![],
            target,
            tolerance,
            min_odds,
            max_odds,
            count: 0,
            error: Some(format!("Failed to fetch data: {}", e)),
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
        .unwrap_or_else(|_| "10000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16 integer");

    // Get the directory where the crate is located to find static files
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let static_path = format!("{}/static", manifest_dir);

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
