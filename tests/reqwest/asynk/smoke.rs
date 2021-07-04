use reqwest::get;

use asserhttp::*;

#[tokio::test]
#[stubr::mock("ping.json")]
async fn simple_should_work() {
    get(stubr.uri()).await.unwrap().expect_status_eq(200);
}

#[should_panic]
#[stubr::mock("ping.json")]
#[tokio::test]
async fn simple_should_panic() {
    get(stubr.uri()).await.unwrap().expect_status_eq(100);
}

#[tokio::test]
#[stubr::mock("ping.json")]
async fn simple_should_chain() {
    get(stubr.uri()).await.unwrap().expect_status_eq(200).expect_status_ok();
}

#[tokio::test]
#[stubr::mock("ping.json")]
async fn result_should_work() {
    get(stubr.uri()).await.expect_status_eq(200);
}

#[should_panic]
#[stubr::mock("ping.json")]
#[tokio::test]
async fn result_should_panic() {
    get(stubr.uri()).await.expect_status_eq(100);
}

#[tokio::test]
#[stubr::mock("ping.json")]
async fn result_should_chain() {
    get(stubr.uri()).await.expect_status_eq(200).expect_status_ok();
}