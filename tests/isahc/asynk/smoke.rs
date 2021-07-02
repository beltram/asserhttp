use isahc::get_async;

use asserhttp::*;

#[async_std::test]
#[stubr::mock("ping.json")]
async fn simple_should_work() {
    get_async(stubr.uri()).await.unwrap().expect_status_eq(200);
}

#[should_panic]
#[stubr::mock("ping.json")]
#[async_std::test]
async fn simple_should_panic() {
    get_async(stubr.uri()).await.unwrap().expect_status_eq(100);
}

#[async_std::test]
#[stubr::mock("ping.json")]
async fn simple_should_chain() {
    get_async(stubr.uri()).await.unwrap().expect_status_eq(200).expect_status_ok();
}

#[async_std::test]
#[stubr::mock("ping.json")]
async fn result_should_work() {
    get_async(stubr.uri()).await.expect_status_eq(200);
}

#[should_panic]
#[stubr::mock("ping.json")]
#[async_std::test]
async fn result_should_panic() {
    get_async(stubr.uri()).await.expect_status_eq(100);
}

#[async_std::test]
#[stubr::mock("ping.json")]
async fn result_should_chain() {
    get_async(stubr.uri()).await.expect_status_eq(200).expect_status_ok();
}