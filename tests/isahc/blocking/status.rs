use isahc::get;
use stubr::Stubr;

use asserhttp::*;

#[test]
fn should_assert_status_eq() {
    let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
    get(&srv.uri()).unwrap().assert_status_eq(200);
}

#[test]
#[should_panic]
fn assert_status_eq_should_panic() {
    let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
    get(&srv.uri()).unwrap().assert_status_eq(100);
}

#[test]
fn result_should_assert_status_eq() {
    let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
    get(&srv.uri()).assert_status_eq(200);
}

#[test]
#[should_panic]
fn result_assert_status_eq_should_panic() {
    let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
    get(&srv.uri()).assert_status_eq(100);
}

#[test]
fn should_assert_status_ok() {
    let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
    get(&srv.uri()).unwrap().assert_status_ok();
}

#[test]
#[should_panic]
fn assert_status_ok_should_panic() {
    let srv = Stubr::start_blocking("tests/stubs/status/created.json");
    get(&srv.uri()).unwrap().assert_status_ok();
}

#[test]
fn result_should_assert_status_ok() {
    let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
    get(&srv.uri()).assert_status_ok();
}

#[test]
#[should_panic]
fn result_assert_status_ok_should_panic() {
    let srv = Stubr::start_blocking("tests/stubs/status/created.json");
    get(&srv.uri()).assert_status_ok();
}

#[test]
fn should_assert_created() {
    let srv = Stubr::start_blocking("tests/stubs/status/created.json");
    get(&srv.uri()).unwrap().assert_status_created();
}

#[test]
#[should_panic]
fn assert_created_should_panic() {
    let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
    get(&srv.uri()).unwrap().assert_status_created();
}

#[test]
fn result_should_assert_created() {
    let srv = Stubr::start_blocking("tests/stubs/status/created.json");
    get(&srv.uri()).assert_status_created();
}

#[test]
#[should_panic]
fn result_assert_created_should_panic() {
    let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
    get(&srv.uri()).assert_status_created();
}

#[test]
fn should_assert_accepted() {
    let srv = Stubr::start_blocking("tests/stubs/status/accepted.json");
    get(&srv.uri()).unwrap().assert_status_accepted();
}

#[test]
#[should_panic]
fn assert_accepted_should_panic() {
    let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
    get(&srv.uri()).unwrap().assert_status_accepted();
}

#[test]
fn result_should_assert_accepted() {
    let srv = Stubr::start_blocking("tests/stubs/status/accepted.json");
    get(&srv.uri()).assert_status_accepted();
}

#[test]
#[should_panic]
fn result_assert_accepted_should_panic() {
    let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
    get(&srv.uri()).assert_status_accepted();
}

#[test]
fn should_assert_no_content() {
    let srv = Stubr::start_blocking("tests/stubs/status/no-content.json");
    get(&srv.uri()).unwrap().assert_status_no_content();
}

#[test]
#[should_panic]
fn assert_no_content_should_panic() {
    let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
    get(&srv.uri()).unwrap().assert_status_no_content();
}

#[test]
fn result_should_assert_no_content() {
    let srv = Stubr::start_blocking("tests/stubs/status/no-content.json");
    get(&srv.uri()).assert_status_no_content();
}

#[test]
#[should_panic]
fn result_assert_no_content_should_panic() {
    let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
    get(&srv.uri()).assert_status_no_content();
}