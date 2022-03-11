use awc::Client;

use asserhttp::*;

mod eq {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("header/one.json")]
    async fn should_expect_header() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_header("x-a", "a");
    }

    #[actix_web::test]
    #[stubr::mock("header/one.json")]
    async fn should_expect_header_ignoring_case() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_header("X-A", "a");
    }

    #[should_panic(expected = "expected header 'x-a' to be equal to 'A' but was 'a'")]
    #[stubr::mock("header/one.json")]
    #[actix_web::test]
    async fn expect_header_value_should_be_case_sensitive() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_header("x-a", "A");
    }

    #[actix_web::test]
    #[stubr::mock("header/many.json")]
    async fn should_expect_many_header() {
        Client::default().get(stubr.uri()).send().await.unwrap()
            .expect_header("x-a", "a")
            .expect_header("x-b", "b");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/one.json")]
    #[actix_web::test]
    async fn expect_header_should_panic_when_wrong_key() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_header("x-b", "a");
    }

    #[should_panic(expected = "expected header 'x-a' to be equal to 'b' but was 'a'")]
    #[stubr::mock("header/one.json")]
    #[actix_web::test]
    async fn expect_header_should_panic_when_wrong_value() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_header("x-a", "b");
    }

    #[should_panic(expected = "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.")]
    #[stubr::mock("header/multi.json")]
    #[actix_web::test]
    async fn expect_header_should_panic_when_multi_valued() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_header("x-m", "a");
    }

    #[actix_web::test]
    #[stubr::mock("header/one.json")]
    async fn result_should_expect_header() {
        Client::default().get(stubr.uri()).send().await.expect_header("x-a", "a");
    }

    #[actix_web::test]
    #[stubr::mock("header/many.json")]
    async fn result_should_expect_many_header() {
        Client::default().get(stubr.uri()).send().await
            .expect_header("x-a", "a")
            .expect_header("x-b", "b");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/one.json")]
    #[actix_web::test]
    async fn result_expect_header_should_panic_when_wrong_key() {
        Client::default().get(stubr.uri()).send().await.expect_header("x-b", "a");
    }

    #[should_panic(expected = "expected header 'x-a' to be equal to 'b' but was 'a'")]
    #[stubr::mock("header/one.json")]
    #[actix_web::test]
    async fn result_expect_header_should_panic_when_wrong_value() {
        Client::default().get(stubr.uri()).send().await.expect_header("x-a", "b");
    }

    #[should_panic(expected = "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.")]
    #[stubr::mock("header/multi.json")]
    #[actix_web::test]
    async fn result_expect_header_should_panic_when_multi_valued() {
        Client::default().get(stubr.uri()).send().await.expect_header("x-m", "a");
    }
}

mod multi {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("header/multi.json")]
    async fn expect_headers_should_expect_multi_headers() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_headers("x-m", ["a", "b"]);
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/multi.json")]
    #[actix_web::test]
    async fn expect_headers_should_fail_when_key_missing() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_headers("x-b", ["a", "b"]);
    }

    #[should_panic(expected = "expected header 'x-m' to contain values '[\"a\"]' but contained '[\"a\", \"b\"]'")]
    #[stubr::mock("header/multi.json")]
    #[actix_web::test]
    async fn expect_headers_should_fail_when_one_value_missing() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_headers("x-m", ["a"]);
    }

    #[should_panic(expected = "expected header 'x-m' to contain values '[\"a\", \"c\"]' but contained '[\"a\", \"b\"]'")]
    #[stubr::mock("header/multi.json")]
    #[actix_web::test]
    async fn expect_headers_should_fail_when_one_value_not_eq() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_headers("x-m", ["a", "c"]);
    }

    #[should_panic(expected = "expected header 'x-m' to contain values '[\"a\", \"b\", \"c\"]' but contained '[\"a\", \"b\"]'")]
    #[stubr::mock("header/multi.json")]
    #[actix_web::test]
    async fn expect_headers_should_fail_when_one_expected_value_missing() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_headers("x-m", ["a", "b", "c"]);
    }

    #[should_panic(expected = "no value expected for header 'x-m'. Use 'expect_header_present' instead")]
    #[stubr::mock("header/multi.json")]
    #[actix_web::test]
    async fn expect_headers_should_fail_when_no_value_expected() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_headers("x-m", []);
    }

    #[actix_web::test]
    #[stubr::mock("header/multi.json")]
    async fn result_expect_headers_should_expect_multi_headers() {
        Client::default().get(stubr.uri()).send().await.expect_headers("x-m", ["a", "b"]);
    }
}

mod present {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("header/one.json")]
    async fn should_expect_header_present() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_header_present("x-a");
    }

    #[actix_web::test]
    #[stubr::mock("header/one.json")]
    async fn should_expect_header_present_ignoring_case() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_header_present("X-A");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/one.json")]
    #[actix_web::test]
    async fn expect_header_present_should_fail_when_absent() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_header_present("x-b");
    }

    #[actix_web::test]
    #[stubr::mock("header/one.json")]
    async fn result_should_expect_header_present() {
        Client::default().get(stubr.uri()).send().await.expect_header_present("x-a");
    }

    #[actix_web::test]
    #[stubr::mock("header/one.json")]
    async fn result_should_expect_header_present_ignoring_case() {
        Client::default().get(stubr.uri()).send().await.expect_header_present("X-A");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/one.json")]
    #[actix_web::test]
    async fn result_expect_header_present_should_fail_when_absent() {
        Client::default().get(stubr.uri()).send().await.expect_header_present("x-b");
    }
}

mod absent {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("header/one.json")]
    async fn should_expect_header_absent() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_header_absent("x-b");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[stubr::mock("header/one.json")]
    #[actix_web::test]
    async fn expect_header_absent_should_fail_when_absent() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_header_absent("x-a");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[stubr::mock("header/one.json")]
    #[actix_web::test]
    async fn expect_header_absent_should_ignore_case() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_header_absent("X-A");
    }

    #[actix_web::test]
    #[stubr::mock("header/one.json")]
    async fn result_should_expect_header_absent() {
        Client::default().get(stubr.uri()).send().await.expect_header_absent("x-b");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[stubr::mock("header/one.json")]
    #[actix_web::test]
    async fn result_expect_header_absent_should_fail_when_absent() {
        Client::default().get(stubr.uri()).send().await.expect_header_absent("x-a");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[stubr::mock("header/one.json")]
    #[actix_web::test]
    async fn result_expect_header_absent_should_ignore_case() {
        Client::default().get(stubr.uri()).send().await.expect_header_absent("X-A");
    }
}

mod json {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("header/json.json")]
    async fn should_expect_content_type_json() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_content_type_json();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'")]
    #[stubr::mock("header/xml.json")]
    #[actix_web::test]
    async fn expect_header_should_panic_when_wrong_value() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_content_type_json();
    }

    #[actix_web::test]
    #[stubr::mock("header/json.json")]
    async fn result_should_expect_content_type_json() {
        Client::default().get(stubr.uri()).send().await.expect_content_type_json();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'")]
    #[stubr::mock("header/xml.json")]
    #[actix_web::test]
    async fn result_expect_header_should_panic_when_wrong_value() {
        Client::default().get(stubr.uri()).send().await.expect_content_type_json();
    }
}

mod text {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("header/text.json")]
    async fn should_expect_content_type_text() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_content_type_text();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'")]
    #[stubr::mock("header/xml.json")]
    #[actix_web::test]
    async fn expect_header_should_panic_when_wrong_value() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_content_type_text();
    }

    #[actix_web::test]
    #[stubr::mock("header/text.json")]
    async fn result_should_expect_content_type_text() {
        Client::default().get(stubr.uri()).send().await.expect_content_type_text();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'")]
    #[stubr::mock("header/xml.json")]
    #[actix_web::test]
    async fn result_expect_header_should_panic_when_wrong_value() {
        Client::default().get(stubr.uri()).send().await.expect_content_type_text();
    }
}