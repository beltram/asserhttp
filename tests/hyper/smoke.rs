use hyper::Client;

use asserhttp::*;

#[tokio::test]
#[stubr::mock("ping.json")]
async fn simple_should_work() {
    Client::new().get(stubr.uri().parse().unwrap()).await.unwrap().expect_status_eq(200);
}

#[should_panic]
#[stubr::mock("ping.json")]
#[tokio::test]
async fn simple_should_panic() {
    Client::new().get(stubr.uri().parse().unwrap()).await.unwrap().expect_status_eq(100);
}

#[tokio::test]
#[stubr::mock("ping.json")]
async fn simple_should_chain() {
    Client::new().get(stubr.uri().parse().unwrap()).await.unwrap().expect_status_eq(200).expect_status_eq(200);
}

#[tokio::test]
#[stubr::mock("ping.json")]
async fn result_should_work() {
    Client::new().get(stubr.uri().parse().unwrap()).await.expect_status_eq(200);
}

#[should_panic]
#[stubr::mock("ping.json")]
#[tokio::test]
async fn result_should_panic() {
    Client::new().get(stubr.uri().parse().unwrap()).await.expect_status_eq(100);
}

#[tokio::test]
#[stubr::mock("ping.json")]
async fn result_should_chain() {
    Client::new().get(stubr.uri().parse().unwrap()).await.expect_status_eq(200).expect_status_eq(200);
}