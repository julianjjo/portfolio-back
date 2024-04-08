use crate::{models::contact_me_model::ContactMe, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{serde::json::Json, State};
use rocket_cors::{Guard, Responder};

#[post("/contact_me", data = "<new_contact_me>")]
pub fn create_contact_me<'a>(
    cors: Guard<'_>,
    db: &'a State<MongoRepo>,
    new_contact_me: Json<ContactMe>,
) -> Responder<Json<InsertOneResult>>{
    let data = ContactMe {
        id: None,
        email: new_contact_me.email.to_owned(),
        subject: new_contact_me.subject.to_owned(),
        message: new_contact_me.message.to_owned(),
    };
    let contact_me_detail = db.create_contact_me(data);

    cors.responder(Json(contact_me_detail.unwrap()))
}