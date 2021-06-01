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
fn should_assert_status_ok() {
    let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
    get(&srv.uri()).unwrap().assert_status_ok();
}

#[test]
fn result_should_assert_status_ok() {
    let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
    get(&srv.uri()).assert_status_ok();
}

#[test]
fn should_assert_created() {
    let srv = Stubr::start_blocking("tests/stubs/status/created.json");
    get(&srv.uri()).unwrap().assert_status_created();
}

#[test]
fn result_should_assert_created() {
    let srv = Stubr::start_blocking("tests/stubs/status/created.json");
    get(&srv.uri()).assert_status_created();
}

#[test]
fn should_assert_accepted() {
    let srv = Stubr::start_blocking("tests/stubs/status/accepted.json");
    get(&srv.uri()).unwrap().assert_status_accepted();
}

#[test]
fn result_should_assert_accepted() {
    let srv = Stubr::start_blocking("tests/stubs/status/accepted.json");
    get(&srv.uri()).assert_status_accepted();
}

#[test]
fn should_assert_no_content() {
    let srv = Stubr::start_blocking("tests/stubs/status/no-content.json");
    get(&srv.uri()).unwrap().assert_status_no_content();
}

#[test]
fn result_should_assert_no_content() {
    let srv = Stubr::start_blocking("tests/stubs/status/no-content.json");
    get(&srv.uri()).assert_status_no_content();
}