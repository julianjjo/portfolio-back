pub mod models;
pub mod repository;
pub mod api;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket::http::Method;

#[macro_use]
extern crate rocket;
use api::contact_me_api::create_contact_me;
use repository::mongodb_repo::MongoRepo;
use rocket::{get, http::Status, serde::json::Json, Request};

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("My portfolio is at https://julian-dev.dev/")))
}

#[catch(500)]
fn internal_error(req: &Request) -> String {
    format!("Internal Server Error: {}", req.uri())
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&["https://julian-dev.dev"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();
    let db = MongoRepo::init();
    rocket::build().manage(db).manage(cors)
        .register("/", catchers![internal_error])
        .mount("/", routes![hello, create_contact_me])
}