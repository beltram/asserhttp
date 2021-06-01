use isahc::get;
use stubr::Stubr;

use asserhttp::*;

#[test]
fn simple_should_work() {
    let srv = Stubr::start_blocking("tests/stubs/ping.json");
    get(&srv.uri()).unwrap().assert_status_eq(200);
}

#[test]
fn simple_should_chain() {
    let srv = Stubr::start_blocking("tests/stubs/ping.json");
    get(&srv.uri()).unwrap().assert_status_eq(200).assert_status_eq(200);
}

#[test]
fn result_should_work() {
    let srv = Stubr::start_blocking("tests/stubs/ping.json");
    get(&srv.uri()).assert_status_eq(200);
}

#[test]
fn result_should_chain() {
    let srv = Stubr::start_blocking("tests/stubs/ping.json");
    get(&srv.uri()).assert_status_eq(200).assert_status_eq(200);
}