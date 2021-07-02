use isahc::get_async;

use asserhttp::*;

mod eq {
    use super::*;

    #[async_std::test]
    #[stubr::mock("header/one.json")]
    async fn should_expect_header() {
        get_async(stubr.uri()).await.unwrap().expect_header("x-a", "a");
    }

    #[async_std::test]
    #[stubr::mock("header/one.json")]
    async fn should_expect_header_ignoring_case() {
        get_async(stubr.uri()).await.unwrap().expect_header("X-A", "a");
    }


    #[should_panic(expected = "expected header 'x-a' to be equal to 'A' but was 'a'")]
    #[stubr::mock("header/one.json")]
    #[async_std::test]
    async fn expect_header_value_should_be_case_sensitive() {
        get_async(stubr.uri()).await.unwrap().expect_header("x-a", "A");
    }

    #[async_std::test]
    #[stubr::mock("header/many.json")]
    async fn should_expect_many_header() {
        get_async(stubr.uri()).await.unwrap()
            .expect_header("x-a", "a")
            .expect_header("x-b", "b");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/one.json")]
    #[async_std::test]
    async fn expect_header_should_panic_when_wrong_key() {
        get_async(stubr.uri()).await.unwrap().expect_header("x-b", "a");
    }

    #[should_panic(expected = "expected header 'x-a' to be equal to 'b' but was 'a'")]
    #[stubr::mock("header/one.json")]
    #[async_std::test]
    async fn expect_header_should_panic_when_wrong_value() {
        get_async(stubr.uri()).await.unwrap().expect_header("x-a", "b");
    }

    #[should_panic(expected = "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.")]
    #[stubr::mock("header/multi.json")]
    #[async_std::test]
    async fn expect_header_should_panic_when_multi_valued() {
        get_async(stubr.uri()).await.unwrap().expect_header("x-m", "a");
    }

    #[async_std::test]
    #[stubr::mock("header/one.json")]
    async fn result_should_expect_header() {
        get_async(stubr.uri()).await.expect_header("x-a", "a");
    }

    #[async_std::test]
    #[stubr::mock("header/many.json")]
    async fn result_should_expect_many_header() {
        get_async(stubr.uri()).await
            .expect_header("x-a", "a")
            .expect_header("x-b", "b");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/one.json")]
    #[async_std::test]
    async fn result_expect_header_should_panic_when_wrong_key() {
        get_async(stubr.uri()).await.expect_header("x-b", "a");
    }

    #[should_panic(expected = "expected header 'x-a' to be equal to 'b' but was 'a'")]
    #[stubr::mock("header/one.json")]
    #[async_std::test]
    async fn result_expect_header_should_panic_when_wrong_value() {
        get_async(stubr.uri()).await.expect_header("x-a", "b");
    }

    #[should_panic(expected = "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.")]
    #[stubr::mock("header/multi.json")]
    #[async_std::test]
    async fn result_expect_header_should_panic_when_multi_valued() {
        get_async(stubr.uri()).await.expect_header("x-m", "a");
    }
}

mod present {
    use super::*;

    #[async_std::test]
    #[stubr::mock("header/one.json")]
    async fn should_expect_header_present() {
        get_async(stubr.uri()).await.unwrap().expect_header_present("x-a");
    }

    #[async_std::test]
    #[stubr::mock("header/one.json")]
    async fn should_expect_header_present_ignoring_case() {
        get_async(stubr.uri()).await.unwrap().expect_header_present("X-A");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/one.json")]
    #[async_std::test]
    async fn expect_header_present_should_fail_when_absent() {
        get_async(stubr.uri()).await.unwrap().expect_header_present("x-b");
    }

    #[async_std::test]
    #[stubr::mock("header/one.json")]
    async fn result_should_expect_header_present() {
        get_async(stubr.uri()).await.expect_header_present("x-a");
    }

    #[async_std::test]
    #[stubr::mock("header/one.json")]
    async fn result_should_expect_header_present_ignoring_case() {
        get_async(stubr.uri()).await.expect_header_present("X-A");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/one.json")]
    #[async_std::test]
    async fn result_expect_header_present_should_fail_when_absent() {
        get_async(stubr.uri()).await.expect_header_present("x-b");
    }
}

mod absent {
    use super::*;

    #[async_std::test]
    #[stubr::mock("header/one.json")]
    async fn should_expect_header_absent() {
        get_async(stubr.uri()).await.unwrap().expect_header_absent("x-b");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[stubr::mock("header/one.json")]
    #[async_std::test]
    async fn expect_header_absent_should_fail_when_absent() {
        get_async(stubr.uri()).await.unwrap().expect_header_absent("x-a");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[stubr::mock("header/one.json")]
    #[async_std::test]
    async fn expect_header_absent_should_ignore_case() {
        get_async(stubr.uri()).await.unwrap().expect_header_absent("X-A");
    }

    #[async_std::test]
    #[stubr::mock("header/one.json")]
    async fn result_should_expect_header_absent() {
        get_async(stubr.uri()).await.expect_header_absent("x-b");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[stubr::mock("header/one.json")]
    #[async_std::test]
    async fn result_expect_header_absent_should_fail_when_absent() {
        get_async(stubr.uri()).await.expect_header_absent("x-a");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[stubr::mock("header/one.json")]
    #[async_std::test]
    async fn result_expect_header_absent_should_ignore_case() {
        get_async(stubr.uri()).await.expect_header_absent("X-A");
    }
}

mod json {
    use super::*;

    #[async_std::test]
    #[stubr::mock("header/json.json")]
    async fn should_expect_content_type_json() {
        get_async(stubr.uri()).await.unwrap().expect_content_type_json();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'")]
    #[stubr::mock("header/xml.json")]
    #[async_std::test]
    async fn expect_header_should_panic_when_wrong_value() {
        get_async(stubr.uri()).await.unwrap().expect_content_type_json();
    }

    #[async_std::test]
    #[stubr::mock("header/json.json")]
    async fn result_should_expect_content_type_json() {
        get_async(stubr.uri()).await.expect_content_type_json();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'")]
    #[stubr::mock("header/xml.json")]
    #[async_std::test]
    async fn result_expect_header_should_panic_when_wrong_value() {
        get_async(stubr.uri()).await.expect_content_type_json();
    }
}

mod text {
    use super::*;

    #[async_std::test]
    #[stubr::mock("header/text.json")]
    async fn should_expect_content_type_text() {
        get_async(stubr.uri()).await.unwrap().expect_content_type_text();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'")]
    #[stubr::mock("header/xml.json")]
    #[async_std::test]
    async fn expect_header_should_panic_when_wrong_value() {
        get_async(stubr.uri()).await.unwrap().expect_content_type_text();
    }

    #[async_std::test]
    #[stubr::mock("header/text.json")]
    async fn result_should_expect_content_type_text() {
        get_async(stubr.uri()).await.expect_content_type_text();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'")]
    #[stubr::mock("header/xml.json")]
    #[async_std::test]
    async fn result_expect_header_should_panic_when_wrong_value() {
        get_async(stubr.uri()).await.expect_content_type_text();
    }
}