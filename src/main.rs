pub mod models;
pub mod repository;
pub mod api;

#[macro_use]
extern crate rocket;
use api::contact_me_api::create_contact_me;
use repository::mongodb_repo::MongoRepo;
use rocket::{get, http::Status, serde::json::Json, http::Header, Response, Request};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "https://julian-dev.dev"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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

    let db = MongoRepo::init();
    rocket::build().attach(CORS).manage(db).register("/", catchers![internal_error])
        .mount("/", routes![hello])
        .mount("/", routes![create_contact_me])
}