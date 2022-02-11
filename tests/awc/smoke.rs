use actix_rt::System;
use awc::Client;

use asserhttp::*;

#[test]
#[stubr::mock("ping.json")]
fn simple_should_work() {
    System::new("test").block_on(async move {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_eq(200);
    });
}

#[should_panic]
#[stubr::mock("ping.json")]
#[test]
fn simple_should_panic() {
    System::new("test").block_on(async move {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_eq(100);
    });
}

#[test]
#[stubr::mock("ping.json")]
fn simple_should_chain() {
    System::new("test").block_on(async move {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_eq(200).expect_status_eq(200);
    });
}

#[test]
#[stubr::mock("ping.json")]
fn result_should_work() {
    System::new("test").block_on(async move {
        Client::default().get(stubr.uri()).send().await.expect_status_eq(200);
    });
}

#[should_panic]
#[stubr::mock("ping.json")]
#[test]
fn result_should_panic() {
    System::new("test").block_on(async move {
        Client::default().get(stubr.uri()).send().await.expect_status_eq(100);
    });
}

#[test]
#[stubr::mock("ping.json")]
fn result_should_chain() {
    System::new("test").block_on(async move {
        Client::default().get(stubr.uri()).send().await.expect_status_eq(200).expect_status_eq(200);
    });
}