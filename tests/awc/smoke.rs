use awc::Client;

use asserhttp::*;

#[actix_web::test]
#[stubr::mock("ping.json")]
async fn simple_should_work() {
    Client::default().get(stubr.uri()).send().await.unwrap().expect_status_eq(200);
}

#[should_panic]
#[stubr::mock("ping.json")]
#[actix_web::test]
async fn simple_should_panic() {
    Client::default().get(stubr.uri()).send().await.unwrap().expect_status_eq(100);
}

#[actix_web::test]
#[stubr::mock("ping.json")]
async fn simple_should_chain() {
    Client::default().get(stubr.uri()).send().await.unwrap().expect_status_eq(200).expect_status_eq(200);
}

#[actix_web::test]
#[stubr::mock("ping.json")]
async fn result_should_work() {
    Client::default().get(stubr.uri()).send().await.expect_status_eq(200);
}

#[should_panic]
#[stubr::mock("ping.json")]
#[actix_web::test]
async fn result_should_panic() {
    Client::default().get(stubr.uri()).send().await.expect_status_eq(100);
}

#[actix_web::test]
#[stubr::mock("ping.json")]
async fn result_should_chain() {
    Client::default().get(stubr.uri()).send().await.expect_status_eq(200).expect_status_eq(200);
}