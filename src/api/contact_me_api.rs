use crate::{models::contact_me_model::ContactMe, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/contact_me", data = "<new_contact_me>")]
pub fn create_contact_me(
    db: &State<MongoRepo>,
    new_contact_me: Json<ContactMe>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = ContactMe {
        id: None,
        email: new_contact_me.email.to_owned(),
        subject: new_contact_me.subject.to_owned(),
        message: new_contact_me.message.to_owned(),
    };
    let contact_me_detail = db.create_contact_me(data);
    match contact_me_detail {
        Ok(contact_me) => Ok(Json(contact_me)),
        Err(_) => Err(rocket::http::Status::InternalServerError),
    }
}