use isahc::get;

use asserhttp::*;

#[test]
#[stubr::mock("ping.json")]
fn simple_should_work() {
    get(stubr.uri()).unwrap().expect_status_eq(200);
}

#[should_panic]
#[stubr::mock("ping.json")]
#[test]
fn simple_should_panic() {
    get(stubr.uri()).unwrap().expect_status_eq(100);
}

#[test]
#[stubr::mock("ping.json")]
fn simple_should_chain() {
    get(stubr.uri()).unwrap().expect_status_eq(200).expect_status_eq(200);
}

#[test]
#[stubr::mock("ping.json")]
fn result_should_work() {
    get(stubr.uri()).expect_status_eq(200);
}

#[should_panic]
#[stubr::mock("ping.json")]
#[test]
fn result_should_panic() {
    get(stubr.uri()).expect_status_eq(100);
}

#[test]
#[stubr::mock("ping.json")]
fn result_should_chain() {
    get(stubr.uri()).expect_status_eq(200).expect_status_eq(200);
}