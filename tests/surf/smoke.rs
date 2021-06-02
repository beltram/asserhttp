use stubr::Stubr;
use surf::get;

use asserhttp::*;

#[async_std::test]
async fn simple_should_work() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get(&srv.uri()).await.unwrap().expect_status_eq(200);
}

#[should_panic]
#[async_std::test]
async fn simple_should_panic() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get(&srv.uri()).await.unwrap().expect_status_eq(100);
}

#[async_std::test]
async fn simple_should_chain() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get(&srv.uri()).await.unwrap().expect_status_eq(200).expect_status_eq(200);
}

#[async_std::test]
async fn result_should_work() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get(&srv.uri()).await.expect_status_eq(200);
}

#[should_panic]
#[async_std::test]
async fn result_should_panic() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get(&srv.uri()).await.expect_status_eq(100);
}

#[async_std::test]
async fn result_should_chain() {
    let srv = Stubr::start("tests/stubs/ping.json").await;
    get(&srv.uri()).await.expect_status_eq(200).expect_status_eq(200);
}