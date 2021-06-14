use isahc::get_async;
use serde_json::json;
use stubr::Stubr;

use asserhttp::*;

use super::super::super::common::Body;

mod json {
    use super::*;

    #[async_std::test]
    async fn should_expect_raw_body_json() {
        let srv = Stubr::start("tests/stubs/body/json/value.json").await;
        get_async(&srv.uri()).await.unwrap().expect_body_json(json!({"a": "b"}));
    }

    #[async_std::test]
    async fn should_expect_struct_body_json() {
        let srv = Stubr::start("tests/stubs/body/json/value.json").await;
        get_async(&srv.uri()).await.unwrap().expect_body_json(Body { a: String::from("b") });
    }

    #[async_std::test]
    #[should_panic(expected = "expected json body Object({\"a\": String(\"b\")}) to be equal to Object({\"a\": String(\"c\")}) but was not")]
    async fn expect_body_json_should_fail_when_not_equal() {
        let srv = Stubr::start("tests/stubs/body/json/value.json").await;
        get_async(&srv.uri()).await.unwrap().expect_body_json(json!({"a": "c"}));
    }

    #[async_std::test]
    #[should_panic(expected = "expected json body Object({\"a\": String(\"b\")}) but no response body was present")]
    async fn expect_body_json_should_fail_when_missing() {
        let srv = Stubr::start("tests/stubs/body/json/missing.json").await;
        get_async(&srv.uri()).await.unwrap().expect_body_json(json!({"a": "b"}));
    }

    #[async_std::test]
    async fn result_should_expect_raw_body_json() {
        let srv = Stubr::start("tests/stubs/body/json/value.json").await;
        get_async(&srv.uri()).await.expect_body_json(json!({"a": "b"}));
    }

    #[async_std::test]
    async fn result_should_expect_struct_body_json() {
        let srv = Stubr::start("tests/stubs/body/json/value.json").await;
        get_async(&srv.uri()).await.expect_body_json(Body { a: String::from("b") });
    }

    #[async_std::test]
    #[should_panic(expected = "expected json body Object({\"a\": String(\"b\")}) to be equal to Object({\"a\": String(\"c\")}) but was not")]
    async fn result_expect_body_json_should_fail_when_not_equal() {
        let srv = Stubr::start("tests/stubs/body/json/value.json").await;
        get_async(&srv.uri()).await.expect_body_json(json!({"a": "c"}));
    }

    #[async_std::test]
    #[should_panic(expected = "expected json body Object({\"a\": String(\"b\")}) but no response body was present")]
    async fn result_expect_body_json_should_fail_when_missing() {
        let srv = Stubr::start("tests/stubs/body/json/missing.json").await;
        get_async(&srv.uri()).await.expect_body_json(json!({"a": "b"}));
    }
}

mod text {
    use super::*;

    #[async_std::test]
    async fn should_expect_body_text() {
        let srv = Stubr::start("tests/stubs/body/text/value.json").await;
        get_async(&srv.uri()).await.unwrap().expect_body_text("abcd");
    }

    #[async_std::test]
    #[should_panic(expected = "expected text body 'abcd' to be equal to 'dcab' but was not")]
    async fn expect_body_text_should_fail_when_not_equal() {
        let srv = Stubr::start("tests/stubs/body/text/value.json").await;
        get_async(&srv.uri()).await.unwrap().expect_body_text("dcab");
    }

    #[async_std::test]
    #[should_panic(expected = "expected text body 'abcd' but no response body was present")]
    async fn expect_body_text_should_fail_when_missing() {
        let srv = Stubr::start("tests/stubs/body/text/missing.json").await;
        get_async(&srv.uri()).await.unwrap().expect_body_text("abcd");
    }

    #[async_std::test]
    async fn result_should_expect_body_text() {
        let srv = Stubr::start("tests/stubs/body/text/value.json").await;
        get_async(&srv.uri()).await.expect_body_text("abcd");
    }

    #[async_std::test]
    #[should_panic(expected = "expected text body 'abcd' to be equal to 'dcab' but was not")]
    async fn result_expect_body_text_should_fail_when_not_equal() {
        let srv = Stubr::start("tests/stubs/body/text/value.json").await;
        get_async(&srv.uri()).await.expect_body_text("dcab");
    }

    #[async_std::test]
    #[should_panic(expected = "expected text body 'abcd' but no response body was present")]
    async fn result_expect_body_text_should_fail_when_missing() {
        let srv = Stubr::start("tests/stubs/body/text/missing.json").await;
        get_async(&srv.uri()).await.expect_body_text("abcd");
    }
}