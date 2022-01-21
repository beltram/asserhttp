use rocket::{get, http::Status, local::blocking::Client, routes};

use asserhttp::*;

#[test]
fn simple_should_work() {
    #[get("/")]
    fn ok() -> Status { Status::Ok }
    let client = Client::tracked(rocket::build().mount("/", routes![ok])).unwrap();
    client.get("/").dispatch().expect_status_eq(200);
}

#[should_panic]
#[test]
fn simple_should_panic() {
    #[get("/")]
    fn ok() -> Status { Status::Ok }
    let client = Client::tracked(rocket::build().mount("/", routes![ok])).unwrap();
    client.get("/").dispatch().expect_status_eq(100);
}

#[test]
fn simple_should_chain() {
    #[get("/")]
    fn ok() -> Status { Status::Ok }
    let client = Client::tracked(rocket::build().mount("/", routes![ok])).unwrap();
    client.get("/").dispatch().expect_status_eq(200).expect_status_eq(200);
}