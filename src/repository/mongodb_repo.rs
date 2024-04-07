use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
    sync::{Client, Collection},
};
use crate::models::contact_me_model::ContactMe;

pub struct MongoRepo {
    col: Collection<ContactMe>,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("portfolio");
        let col: Collection<ContactMe> = db.collection("ContactMe");
        MongoRepo { col }
    }

    pub fn create_user(&self, new_contact_me: ContactMe) -> Result<InsertOneResult, Error> {
        let new_doc = ContactMe {
            id: None,
            email: new_contact_me.email,
            subject: new_contact_me.subject,
            message: new_contact_me.message,
        };
        let user = self
            .col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
}