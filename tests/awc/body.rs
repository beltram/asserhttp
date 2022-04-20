use awc::Client;
use serde_json::{json, Value};

use asserhttp::*;

use crate::Body;

mod json {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("body/json/value.json")]
    async fn should_expect_raw_body_json() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    }

    #[actix_web::test]
    #[stubr::mock("body/json/value.json")]
    async fn should_expect_struct_body_json() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_json(|b: Body| assert_eq!(b, Body { a: String::from("b") }));
    }

    #[should_panic]
    #[stubr::mock("body/json/value.json")]
    #[actix_web::test]
    async fn expect_body_json_should_fail_when_closure_fails() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "c"})));
    }

    #[actix_web::test]
    #[stubr::mock("body/json/value.json")]
    async fn result_should_expect_raw_body_json() {
        Client::default().get(stubr.uri()).send().await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    }

    #[actix_web::test]
    #[stubr::mock("body/json/value.json")]
    async fn result_should_expect_struct_body_json() {
        Client::default().get(stubr.uri()).send().await.expect_body_json(|b: Body| assert_eq!(b, Body { a: String::from("b") }));
    }

    #[should_panic]
    #[stubr::mock("body/json/value.json")]
    #[actix_web::test]
    async fn result_expect_body_json_should_fail_when_closure_fails() {
        Client::default().get(stubr.uri()).send().await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "c"})));
    }
}

mod json_eq {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("body/json/value.json")]
    async fn should_expect_raw_body_json() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_json_eq(json!({"a": "b"}));
    }

    #[actix_web::test]
    #[stubr::mock("body/json/value.json")]
    async fn should_expect_struct_body_json() {}

    #[should_panic(expected = "expected json body '{\"a\":\"b\"}' to be equal to '{\"a\":\"c\"}' but was not")]
    #[stubr::mock("body/json/value.json")]
    #[actix_web::test]
    async fn expect_body_json_should_fail_when_not_equal() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_json_eq(json!({"a": "c"}));
    }

    #[should_panic(expected = "expected a json body but no response body was present")]
    #[stubr::mock("body/json/absent.json")]
    #[actix_web::test]
    async fn expect_body_json_should_fail_when_missing() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_json_eq(json!({"a": "b"}));
    }

    #[actix_web::test]
    #[stubr::mock("body/json/value.json")]
    async fn result_should_expect_raw_body_json() {
        Client::default().get(stubr.uri()).send().await.expect_body_json_eq(json!({"a": "b"}));
    }

    #[actix_web::test]
    #[stubr::mock("body/json/value.json")]
    async fn result_should_expect_struct_body_json() {}

    #[should_panic(expected = "expected json body '{\"a\":\"b\"}' to be equal to '{\"a\":\"c\"}' but was not")]
    #[stubr::mock("body/json/value.json")]
    #[actix_web::test]
    async fn result_expect_body_json_should_fail_when_not_equal() {
        Client::default().get(stubr.uri()).send().await.expect_body_json_eq(json!({"a": "c"}));
    }

    #[should_panic(expected = "expected a json body but no response body was present")]
    #[stubr::mock("body/json/absent.json")]
    #[actix_web::test]
    async fn result_expect_body_json_should_fail_when_missing() {
        Client::default().get(stubr.uri()).send().await.expect_body_json_eq(json!({"a": "b"}));
    }
}

mod text {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("body/text/value.json")]
    async fn should_expect_body_text() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_text(|b| assert_eq!(b, String::from("abcd")));
    }

    #[should_panic]
    #[stubr::mock("body/text/value.json")]
    #[actix_web::test]
    async fn expect_body_text_should_fail_when_closure_fails() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_text(|b| assert_eq!(b, String::from("dcba")));
    }

    #[actix_web::test]
    #[stubr::mock("body/text/value.json")]
    async fn result_should_expect_body_text() {
        Client::default().get(stubr.uri()).send().await.expect_body_text(|b| assert_eq!(b, String::from("abcd")));
    }

    #[should_panic]
    #[stubr::mock("body/text/value.json")]
    #[actix_web::test]
    async fn result_expect_body_text_should_fail_when_closure_fails() {
        Client::default().get(stubr.uri()).send().await.expect_body_text(|b| assert_eq!(b, String::from("dcba")));
    }
}

mod text_eq {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("body/text/value.json")]
    async fn should_expect_body_text_eq() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_text_eq("abcd");
    }

    #[should_panic(expected = "expected text body 'abcd' to be equal to 'dcab' but was not")]
    #[stubr::mock("body/text/value.json")]
    #[actix_web::test]
    async fn expect_body_text_should_fail_when_not_equal() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_text_eq("dcab");
    }

    #[should_panic(expected = "expected a text body but no response body was present")]
    #[stubr::mock("body/text/absent.json")]
    #[actix_web::test]
    async fn expect_body_text_should_fail_when_missing() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_text_eq("abcd");
    }

    #[actix_web::test]
    #[stubr::mock("body/text/value.json")]
    async fn result_should_expect_body_text_eq() {
        Client::default().get(stubr.uri()).send().await.expect_body_text_eq("abcd");
    }

    #[should_panic(expected = "expected text body 'abcd' to be equal to 'dcab' but was not")]
    #[stubr::mock("body/text/value.json")]
    #[actix_web::test]
    async fn result_expect_body_text_should_fail_when_not_equal() {
        Client::default().get(stubr.uri()).send().await.expect_body_text_eq("dcab");
    }

    #[should_panic(expected = "expected a text body but no response body was present")]
    #[stubr::mock("body/text/absent.json")]
    #[actix_web::test]
    async fn result_expect_body_text_should_fail_when_missing() {
        Client::default().get(stubr.uri()).send().await.expect_body_text_eq("abcd");
    }
}

mod regex {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("body/text/value.json")]
    async fn should_expect_body_text_matches() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_text_matches("[a-d]+");
    }

    #[should_panic(expected = "expected text body 'abcd' to match regex '[e-h]+' but did not")]
    #[stubr::mock("body/text/value.json")]
    #[actix_web::test]
    async fn expect_body_text_matches_should_fail_when_does_not_match_regex() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_text_matches("[e-h]+");
    }

    #[should_panic(expected = "expected a text body but no response body was present")]
    #[stubr::mock("body/text/absent.json")]
    #[actix_web::test]
    async fn expect_body_text_should_fail_when_missing() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_text_matches("[a-d]+");
    }

    #[actix_web::test]
    #[stubr::mock("body/text/value.json")]
    async fn result_should_expect_body_text_matches() {
        Client::default().get(stubr.uri()).send().await.expect_body_text_matches("[a-d]+");
    }

    #[should_panic(expected = "expected text body 'abcd' to match regex '[e-h]+' but did not")]
    #[stubr::mock("body/text/value.json")]
    #[actix_web::test]
    async fn result_expect_body_text_matches_should_fail_when_does_not_match_regex() {
        Client::default().get(stubr.uri()).send().await.expect_body_text_matches("[e-h]+");
    }

    #[should_panic(expected = "expected a text body but no response body was present")]
    #[stubr::mock("body/text/absent.json")]
    #[actix_web::test]
    async fn result_expect_body_text_should_fail_when_missing() {
        Client::default().get(stubr.uri()).send().await.expect_body_text_matches("[a-d]+");
    }
}

mod bytes {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("body/bytes/value.json")]
    async fn should_expect_body_bytes() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    }

    #[should_panic]
    #[stubr::mock("body/bytes/value.json")]
    #[actix_web::test]
    async fn expect_body_bytes_should_fail_when_closure_fails() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_bytes(|b| assert_eq!(b, b"dcba"));
    }

    #[actix_web::test]
    #[stubr::mock("body/bytes/value.json")]
    async fn result_should_expect_body_bytes() {
        Client::default().get(stubr.uri()).send().await.expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    }

    #[should_panic]
    #[stubr::mock("body/bytes/value.json")]
    #[actix_web::test]
    async fn result_expect_body_bytes_should_fail_when_closure_fails() {
        Client::default().get(stubr.uri()).send().await.expect_body_bytes(|b| assert_eq!(b, b"dcba"));
    }
}

mod bytes_eq {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("body/bytes/value.json")]
    async fn should_expect_body_bytes_eq() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_bytes_eq(b"abcd");
    }

    #[should_panic(expected = "expected body '[97, 98, 99, 100]' to be equal to '[100, 99, 98, 97]' but was not")]
    #[stubr::mock("body/bytes/value.json")]
    #[actix_web::test]
    async fn expect_body_bytes_should_fail_not_equal() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_bytes_eq(b"dcba");
    }

    #[should_panic(expected = "expected a response body but no response body was present")]
    #[stubr::mock("body/bytes/absent.json")]
    #[actix_web::test]
    async fn expect_body_bytes_should_fail_when_missing() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_bytes_eq(b"abcd");
    }

    #[actix_web::test]
    #[stubr::mock("body/bytes/value.json")]
    async fn result_should_expect_body_bytes_eq() {
        Client::default().get(stubr.uri()).send().await.expect_body_bytes_eq(b"abcd");
    }

    #[should_panic(expected = "expected body '[97, 98, 99, 100]' to be equal to '[100, 99, 98, 97]' but was not")]
    #[stubr::mock("body/bytes/value.json")]
    #[actix_web::test]
    async fn result_expect_body_bytes_should_fail_not_equal() {
        Client::default().get(stubr.uri()).send().await.expect_body_bytes_eq(b"dcba");
    }

    #[should_panic(expected = "expected a response body but no response body was present")]
    #[stubr::mock("body/bytes/absent.json")]
    #[actix_web::test]
    async fn result_expect_body_bytes_should_fail_when_missing() {
        Client::default().get(stubr.uri()).send().await.expect_body_bytes_eq(b"abcd");
    }
}

mod present {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("body/bytes/value.json")]
    async fn should_expect_body_present() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_present();
    }

    #[should_panic(expected = "expected a response body but no response body was present")]
    #[stubr::mock("body/bytes/absent.json")]
    #[actix_web::test]
    async fn expect_body_present_should_fail_when_absent() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_present();
    }

    #[actix_web::test]
    #[stubr::mock("body/bytes/value.json")]
    async fn result_should_expect_body_present() {
        Client::default().get(stubr.uri()).send().await.expect_body_present();
    }

    #[should_panic(expected = "expected a response body but no response body was present")]
    #[stubr::mock("body/bytes/absent.json")]
    #[actix_web::test]
    async fn result_expect_body_present_should_fail_when_absent() {
        Client::default().get(stubr.uri()).send().await.expect_body_present();
    }
}

mod absent {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("body/bytes/absent.json")]
    async fn should_expect_body_absent() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_absent();
    }

    #[should_panic(expected = "expected no response body but a response body was present")]
    #[stubr::mock("body/bytes/value.json")]
    #[actix_web::test]
    async fn expect_body_absent_should_fail_when_present() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_body_absent();
    }

    #[actix_web::test]
    #[stubr::mock("body/bytes/absent.json")]
    async fn result_should_expect_body_absent() {
        Client::default().get(stubr.uri()).send().await.expect_body_absent();
    }

    #[should_panic(expected = "expected no response body but a response body was present")]
    #[stubr::mock("body/bytes/value.json")]
    #[actix_web::test]
    async fn result_expect_body_absent_should_fail_when_present() {
        Client::default().get(stubr.uri()).send().await.expect_body_absent();
    }
}