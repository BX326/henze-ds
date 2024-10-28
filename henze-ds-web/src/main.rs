use henze_ds::{retrieve_henze_data, HenzeInfo};

use rocket::serde::json::Json;
use rocket::response::Redirect;
use rocket::http::Status;
use rocket::fs::FileServer;
use rocket::{self, catch, catchers, get, launch, routes, uri};
use rocket_dyn_templates::Template;
use std::env;

#[get("/")]
async fn process_data_route() -> Result<Json<Vec<HenzeInfo>>, Status> {
    match retrieve_henze_data().await {
        Ok(data) => Ok(Json(data)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to(uri!(process_data_route))
}

#[launch]
fn rocket() -> _ {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "10000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16 integer");

    rocket::build()
        .mount("/", routes![process_data_route])
        // .mount("/static", FileServer::from("./static"))
        // .attach(Template::fairing())
        .register("/", catchers![not_found])
        .configure(rocket::Config {
            port,
            address: "0.0.0.0".parse().unwrap(),
            ..rocket::Config::default()
        })
    }
