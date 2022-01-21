use rocket::{get, routes, http::Status, local::asynchronous::Client};

use asserhttp::*;

#[rocket::async_test]
async fn simple_should_work() {
    #[get("/")]
    fn ok() -> Status { Status::Ok }
    let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
    client.get("/").dispatch().await.expect_status_eq(200);
}

#[should_panic]
#[rocket::async_test]
async fn simple_should_panic() {
    #[get("/")]
    fn ok() -> Status { Status::Ok }
    let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
    client.get("/").dispatch().await.expect_status_eq(100);
}

#[rocket::async_test]
async fn simple_should_chain() {
    #[get("/")]
    fn ok() -> Status { Status::Ok }
    let client = Client::tracked(rocket::build().mount("/", routes![ok])).await.unwrap();
    client.get("/").dispatch().await.expect_status_eq(200).expect_status_eq(200);
}