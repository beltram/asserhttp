use isahc::get_async;
use stubr::Stubr;

use asserhttp::*;

#[async_std::test]
async fn should_assert_status_eq() {
    let srv = Stubr::start("tests/stubs/status/eq.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_eq(200);
}

#[should_panic]
#[async_std::test]
async fn assert_status_eq_should_panic() {
    let srv = Stubr::start("tests/stubs/status/eq.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_eq(100);
}

#[async_std::test]
async fn result_should_assert_status_eq() {
    let srv = Stubr::start("tests/stubs/status/eq.json").await;
    get_async(&srv.uri()).await.assert_status_eq(200);
}

#[should_panic]
#[async_std::test]
async fn result_assert_status_eq_should_panic() {
    let srv = Stubr::start("tests/stubs/status/eq.json").await;
    get_async(&srv.uri()).await.assert_status_eq(100);
}

#[async_std::test]
async fn should_assert_status_ok() {
    let srv = Stubr::start("tests/stubs/status/ok.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_ok();
}

#[should_panic]
#[async_std::test]
async fn assert_status_ok_should_panic() {
    let srv = Stubr::start("tests/stubs/status/created.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_ok();
}

#[async_std::test]
async fn result_should_assert_status_ok() {
    let srv = Stubr::start("tests/stubs/status/ok.json").await;
    get_async(&srv.uri()).await.assert_status_ok();
}

#[should_panic]
#[async_std::test]
async fn result_assert_status_ok_should_panic() {
    let srv = Stubr::start("tests/stubs/status/created.json").await;
    get_async(&srv.uri()).await.assert_status_ok();
}

#[async_std::test]
async fn should_assert_status_created() {
    let srv = Stubr::start("tests/stubs/status/created.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_created();
}

#[should_panic]
#[async_std::test]
async fn assert_status_created_should_panic() {
    let srv = Stubr::start("tests/stubs/status/ok.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_created();
}

#[async_std::test]
async fn result_should_assert_status_created() {
    let srv = Stubr::start("tests/stubs/status/created.json").await;
    get_async(&srv.uri()).await.assert_status_created();
}

#[should_panic]
#[async_std::test]
async fn result_assert_status_created_should_panic() {
    let srv = Stubr::start("tests/stubs/status/ok.json").await;
    get_async(&srv.uri()).await.assert_status_created();
}

#[async_std::test]
async fn should_assert_status_accepted() {
    let srv = Stubr::start("tests/stubs/status/accepted.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_accepted();
}

#[should_panic]
#[async_std::test]
async fn assert_status_accepted_should_panic() {
    let srv = Stubr::start("tests/stubs/status/ok.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_accepted();
}

#[async_std::test]
async fn result_should_assert_status_accepted() {
    let srv = Stubr::start("tests/stubs/status/accepted.json").await;
    get_async(&srv.uri()).await.assert_status_accepted();
}

#[should_panic]
#[async_std::test]
async fn result_assert_status_accepted_should_panic() {
    let srv = Stubr::start("tests/stubs/status/ok.json").await;
    get_async(&srv.uri()).await.assert_status_accepted();
}

#[async_std::test]
async fn should_assert_status_no_content() {
    let srv = Stubr::start("tests/stubs/status/no-content.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_no_content();
}

#[should_panic]
#[async_std::test]
async fn assert_status_no_content_should_panic() {
    let srv = Stubr::start("tests/stubs/status/ok.json").await;
    get_async(&srv.uri()).await.unwrap().assert_status_no_content();
}

#[async_std::test]
async fn result_should_assert_status_no_content() {
    let srv = Stubr::start("tests/stubs/status/no-content.json").await;
    get_async(&srv.uri()).await.assert_status_no_content();
}

#[should_panic]
#[async_std::test]
async fn result_assert_status_no_content_should_panic() {
    let srv = Stubr::start("tests/stubs/status/ok.json").await;
    get_async(&srv.uri()).await.assert_status_no_content();
}