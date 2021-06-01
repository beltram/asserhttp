use stubr::Stubr;
use surf::get;

use asserhttp::*;

#[async_std::test]
async fn should_assert_status_eq() {
    let srv = Stubr::start("tests/stubs/status/eq.json").await;
    get(&srv.uri()).await.unwrap().assert_status_eq(200);
}

#[async_std::test]
async fn result_should_assert_status_eq() {
    let srv = Stubr::start("tests/stubs/status/eq.json").await;
    get(&srv.uri()).await.assert_status_eq(200);
}

#[async_std::test]
async fn should_assert_ok() {
    let srv = Stubr::start("tests/stubs/status/ok.json").await;
    get(&srv.uri()).await.unwrap().assert_ok();
}

#[async_std::test]
async fn result_should_assert_ok() {
    let srv = Stubr::start("tests/stubs/status/ok.json").await;
    get(&srv.uri()).await.assert_ok();
}