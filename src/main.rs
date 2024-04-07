pub mod models;
pub mod repository;
pub mod api;

#[macro_use]
extern crate rocket;
use api::contact_me_api::create_contact_me;
use repository::mongodb_repo::MongoRepo;
use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("My portfolio is at https://julian-dev.dev/")))
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build().manage(db)
        .mount("/", routes![hello])
        .mount("/", routes![create_contact_me])
}