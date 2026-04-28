//! Henze Web Application - Rocket-based web interface for browsing bets.

mod handlers;
mod models;
mod utils;
mod cache;

use rocket::fs::FileServer;
use rocket::{catchers, launch, routes};
use rocket_dyn_templates::Template;
use std::env;

use handlers::{api_bets, api_events, index, internal_prefetch, internal_status, not_found};

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
        .mount("/", routes![index, api_events, api_bets, internal_prefetch, internal_status])
        .mount("/static", FileServer::from(static_path))
        .attach(Template::fairing())
        .register("/", catchers![not_found])
        .configure(rocket::Config {
            port,
            address: "0.0.0.0".parse().unwrap(),
            ..rocket::Config::default()
        })
}
