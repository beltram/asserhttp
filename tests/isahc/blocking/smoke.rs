use isahc::get;
use stubr::Stubr;

use asserhttp::*;

#[test]
fn simple_should_work() {
    let srv = Stubr::start_blocking("tests/stubs/ping.json");
    get(&srv.uri()).unwrap().expect_status_eq(200);
}

#[test]
#[should_panic]
fn simple_should_panic() {
    let srv = Stubr::start_blocking("tests/stubs/ping.json");
    get(&srv.uri()).unwrap().expect_status_eq(100);
}

#[test]
fn simple_should_chain() {
    let srv = Stubr::start_blocking("tests/stubs/ping.json");
    get(&srv.uri()).unwrap().expect_status_eq(200).expect_status_eq(200);
}

#[test]
fn result_should_work() {
    let srv = Stubr::start_blocking("tests/stubs/ping.json");
    get(&srv.uri()).expect_status_eq(200);
}

#[test]
#[should_panic]
fn result_should_panic() {
    let srv = Stubr::start_blocking("tests/stubs/ping.json");
    get(&srv.uri()).expect_status_eq(100);
}

#[test]
fn result_should_chain() {
    let srv = Stubr::start_blocking("tests/stubs/ping.json");
    get(&srv.uri()).expect_status_eq(200).expect_status_eq(200);
}