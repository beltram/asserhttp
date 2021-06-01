use isahc::get;
use stubr::Stubr;

use asserhttp::*;

#[test]
fn should_assert_status_eq() {
    let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
    get(&srv.uri()).unwrap().assert_status_eq(200);
}

#[test]
fn result_should_assert_status_eq() {
    let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
    get(&srv.uri()).assert_status_eq(200);
}

#[test]
fn should_assert_ok() {
    let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
    get(&srv.uri()).unwrap().assert_ok();
}

#[test]
fn result_should_assert_ok() {
    let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
    get(&srv.uri()).assert_ok();
}