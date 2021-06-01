use isahc::get_async;
use stubr::Stubr;

use asserhttp::*;

#[async_std::test]
async fn simple_should_work() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_eq(200);
}

#[should_panic]
#[async_std::test]
async fn simple_should_panic() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_eq(100);
}

#[async_std::test]
async fn simple_should_chain() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_eq(200).assert_status_ok();
}

#[async_std::test]
async fn result_should_work() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get_async(&srv.uri()).await.assert_status_eq(200);
}

#[should_panic]
#[async_std::test]
async fn result_should_panic() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get_async(&srv.uri()).await.assert_status_eq(100);
}

#[async_std::test]
async fn result_should_chain() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get_async(&srv.uri()).await.assert_status_eq(200).assert_status_ok();
}