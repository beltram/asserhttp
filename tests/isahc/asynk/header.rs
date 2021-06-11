use isahc::get_async;
use stubr::Stubr;

use asserhttp::*;

mod eq {
    use super::*;

    #[async_std::test]
    async fn should_expect_header() {
        let srv = Stubr::start("tests/stubs/header/one.json").await;
        get_async(&srv.uri()).await.unwrap().expect_header("x-a", "a");
    }

    #[async_std::test]
    async fn should_expect_many_header() {
        let srv = Stubr::start("tests/stubs/header/many.json").await;
        get_async(&srv.uri()).await.unwrap()
            .expect_header("x-a", "a")
            .expect_header("x-b", "b");
    }

    #[async_std::test]
    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    async fn expect_header_should_panic_when_wrong_key() {
        let srv = Stubr::start("tests/stubs/header/one.json").await;
        get_async(&srv.uri()).await.unwrap().expect_header("x-b", "a");
    }

    #[async_std::test]
    #[should_panic(expected = "expected header 'x-a' to be equal to 'b' but was 'a'")]
    async fn expect_header_should_panic_when_wrong_value() {
        let srv = Stubr::start("tests/stubs/header/one.json").await;
        get_async(&srv.uri()).await.unwrap().expect_header("x-a", "b");
    }

    #[async_std::test]
    #[should_panic(expected = "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.")]
    async fn expect_header_should_panic_when_multi_valued() {
        let srv = Stubr::start("tests/stubs/header/multi.json").await;
        get_async(&srv.uri()).await.unwrap().expect_header("x-m", "a");
    }

    #[async_std::test]
    async fn result_should_expect_header() {
        let srv = Stubr::start("tests/stubs/header/one.json").await;
        get_async(&srv.uri()).await.expect_header("x-a", "a");
    }

    #[async_std::test]
    async fn result_should_expect_many_header() {
        let srv = Stubr::start("tests/stubs/header/many.json").await;
        get_async(&srv.uri()).await
            .expect_header("x-a", "a")
            .expect_header("x-b", "b");
    }

    #[async_std::test]
    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    async fn result_expect_header_should_panic_when_wrong_key() {
        let srv = Stubr::start("tests/stubs/header/one.json").await;
        get_async(&srv.uri()).await.expect_header("x-b", "a");
    }

    #[async_std::test]
    #[should_panic(expected = "expected header 'x-a' to be equal to 'b' but was 'a'")]
    async fn result_expect_header_should_panic_when_wrong_value() {
        let srv = Stubr::start("tests/stubs/header/one.json").await;
        get_async(&srv.uri()).await.expect_header("x-a", "b");
    }

    #[async_std::test]
    #[should_panic(expected = "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.")]
    async fn result_expect_header_should_panic_when_multi_valued() {
        let srv = Stubr::start("tests/stubs/header/multi.json").await;
        get_async(&srv.uri()).await.expect_header("x-m", "a");
    }
}

mod json {
    use super::*;

    #[async_std::test]
    async fn should_expect_content_type_json() {
        let srv = Stubr::start("tests/stubs/header/json.json").await;
        get_async(&srv.uri()).await.unwrap().expect_content_type_json();
    }

    #[async_std::test]
    #[should_panic(expected = "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'")]
    async fn expect_header_should_panic_when_wrong_value() {
        let srv = Stubr::start("tests/stubs/header/xml.json").await;
        get_async(&srv.uri()).await.unwrap().expect_content_type_json();
    }

    #[async_std::test]
    async fn result_should_expect_content_type_json() {
        let srv = Stubr::start("tests/stubs/header/json.json").await;
        get_async(&srv.uri()).await.expect_content_type_json();
    }

    #[async_std::test]
    #[should_panic(expected = "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'")]
    async fn result_expect_header_should_panic_when_wrong_value() {
        let srv = Stubr::start("tests/stubs/header/xml.json").await;
        get_async(&srv.uri()).await.expect_content_type_json();
    }
}

mod text {
    use super::*;

    #[async_std::test]
    async fn should_expect_content_type_text() {
        let srv = Stubr::start("tests/stubs/header/text.json").await;
        get_async(&srv.uri()).await.unwrap().expect_content_type_text();
    }

    #[async_std::test]
    #[should_panic(expected = "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'")]
    async fn expect_header_should_panic_when_wrong_value() {
        let srv = Stubr::start("tests/stubs/header/xml.json").await;
        get_async(&srv.uri()).await.unwrap().expect_content_type_text();
    }

    #[async_std::test]
    async fn result_should_expect_content_type_text() {
        let srv = Stubr::start("tests/stubs/header/text.json").await;
        get_async(&srv.uri()).await.expect_content_type_text();
    }

    #[async_std::test]
    #[should_panic(expected = "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'")]
    async fn result_expect_header_should_panic_when_wrong_value() {
        let srv = Stubr::start("tests/stubs/header/xml.json").await;
        get_async(&srv.uri()).await.expect_content_type_text();
    }
}