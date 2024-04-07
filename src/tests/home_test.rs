use crate::rocket;
use crate::rocket_uri_macro_home;
use rocket::local::blocking::Client;
use rocket::http::Status;
use rocket::uri;

#[test]
fn home_test() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get(uri!(home)).dispatch();
    let expected = "\"My portfolio is at https://julian-dev.dev/\"";
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), expected);
}