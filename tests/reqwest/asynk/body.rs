use reqwest::get;
use serde_json::{json, Value};

use asserhttp::*;

use super::super::super::common::Body;

mod json {
    use super::*;

    #[tokio::test]
    #[stubr::mock("body/json/value.json")]
    async fn should_expect_raw_body_json() {
        get(stubr.uri()).await.unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    }

    #[tokio::test]
    #[stubr::mock("body/json/value.json")]
    async fn should_expect_struct_body_json() {
        get(stubr.uri()).await.unwrap().expect_body_json(|b: Body| assert_eq!(b, Body { a: String::from("b") }));
    }

    #[should_panic]
    #[stubr::mock("body/json/value.json")]
    #[tokio::test]
    async fn expect_body_json_should_fail_when_closure_fails() {
        get(stubr.uri()).await.unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "c"})));
    }

    #[tokio::test]
    #[stubr::mock("body/json/value.json")]
    async fn result_should_expect_raw_body_json() {
        get(stubr.uri()).await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    }

    #[tokio::test]
    #[stubr::mock("body/json/value.json")]
    async fn result_should_expect_struct_body_json() {
        get(stubr.uri()).await.expect_body_json(|b: Body| assert_eq!(b, Body { a: String::from("b") }));
    }

    #[should_panic]
    #[stubr::mock("body/json/value.json")]
    #[tokio::test]
    async fn result_expect_body_json_should_fail_when_closure_fails() {
        get(stubr.uri()).await.expect_body_json(|b: Value| assert_eq!(b, json!({"a": "c"})));
    }
}

mod json_eq {
    use super::*;

    #[tokio::test]
    #[stubr::mock("body/json/value.json")]
    async fn should_expect_raw_body_json() {
        get(stubr.uri()).await.unwrap().expect_body_json_eq(json!({"a": "b"}));
    }

    #[tokio::test]
    #[stubr::mock("body/json/value.json")]
    async fn should_expect_struct_body_json() {
        get(stubr.uri()).await.unwrap().expect_body_json_eq(Body { a: String::from("b") });
    }

    #[should_panic(expected = "expected json body '{\"a\":\"b\"}' to be equal to '{\"a\":\"c\"}' but was not")]
    #[stubr::mock("body/json/value.json")]
    #[tokio::test]
    async fn expect_body_json_should_fail_when_not_equal() {
        get(stubr.uri()).await.unwrap().expect_body_json_eq(json!({"a": "c"}));
    }

    #[should_panic(expected = "expected a json body but no response body was present")]
    #[stubr::mock("body/json/absent.json")]
    #[tokio::test]
    async fn expect_body_json_should_fail_when_missing() {
        get(stubr.uri()).await.unwrap().expect_body_json_eq(json!({"a": "b"}));
    }

    #[tokio::test]
    #[stubr::mock("body/json/value.json")]
    async fn result_should_expect_raw_body_json() {
        get(stubr.uri()).await.expect_body_json_eq(json!({"a": "b"}));
    }

    #[tokio::test]
    #[stubr::mock("body/json/value.json")]
    async fn result_should_expect_struct_body_json() {
        get(stubr.uri()).await.expect_body_json_eq(Body { a: String::from("b") });
    }

    #[should_panic(expected = "expected json body '{\"a\":\"b\"}' to be equal to '{\"a\":\"c\"}' but was not")]
    #[stubr::mock("body/json/value.json")]
    #[tokio::test]
    async fn result_expect_body_json_should_fail_when_not_equal() {
        get(stubr.uri()).await.expect_body_json_eq(json!({"a": "c"}));
    }

    #[should_panic(expected = "expected a json body but no response body was present")]
    #[stubr::mock("body/json/absent.json")]
    #[tokio::test]
    async fn result_expect_body_json_should_fail_when_missing() {
        get(stubr.uri()).await.expect_body_json_eq(json!({"a": "b"}));
    }
}

mod text {
    use super::*;

    #[tokio::test]
    #[stubr::mock("body/text/value.json")]
    async fn should_expect_body_text() {
        get(stubr.uri()).await.unwrap().expect_body_text(|b| assert_eq!(b, String::from("abcd")));
    }

    #[should_panic]
    #[stubr::mock("body/text/value.json")]
    #[tokio::test]
    async fn expect_body_text_should_fail_when_closure_fails() {
        get(stubr.uri()).await.unwrap().expect_body_text(|b| assert_eq!(b, String::from("dcba")));
    }

    #[tokio::test]
    #[stubr::mock("body/text/value.json")]
    async fn result_should_expect_body_text() {
        get(stubr.uri()).await.expect_body_text(|b| assert_eq!(b, String::from("abcd")));
    }

    #[should_panic]
    #[stubr::mock("body/text/value.json")]
    #[tokio::test]
    async fn result_expect_body_text_should_fail_when_closure_fails() {
        get(stubr.uri()).await.expect_body_text(|b| assert_eq!(b, String::from("dcba")));
    }
}

mod text_eq {
    use super::*;

    #[tokio::test]
    #[stubr::mock("body/text/value.json")]
    async fn should_expect_body_text_eq() {
        get(stubr.uri()).await.unwrap().expect_body_text_eq("abcd");
    }

    #[should_panic(expected = "expected text body 'abcd' to be equal to 'dcab' but was not")]
    #[stubr::mock("body/text/value.json")]
    #[tokio::test]
    async fn expect_body_text_should_fail_when_not_equal() {
        get(stubr.uri()).await.unwrap().expect_body_text_eq("dcab");
    }

    #[should_panic(expected = "expected a text body but no response body was present")]
    #[stubr::mock("body/text/absent.json")]
    #[tokio::test]
    async fn expect_body_text_should_fail_when_missing() {
        get(stubr.uri()).await.unwrap().expect_body_text_eq("abcd");
    }

    #[tokio::test]
    #[stubr::mock("body/text/value.json")]
    async fn result_should_expect_body_text_eq() {
        get(stubr.uri()).await.expect_body_text_eq("abcd");
    }

    #[should_panic(expected = "expected text body 'abcd' to be equal to 'dcab' but was not")]
    #[stubr::mock("body/text/value.json")]
    #[tokio::test]
    async fn result_expect_body_text_should_fail_when_not_equal() {
        get(stubr.uri()).await.expect_body_text_eq("dcab");
    }

    #[should_panic(expected = "expected a text body but no response body was present")]
    #[stubr::mock("body/text/absent.json")]
    #[tokio::test]
    async fn result_expect_body_text_should_fail_when_missing() {
        get(stubr.uri()).await.expect_body_text_eq("abcd");
    }
}

mod regex {
    use super::*;

    #[tokio::test]
    #[stubr::mock("body/text/value.json")]
    async fn should_expect_body_text_matches() {
        get(stubr.uri()).await.unwrap().expect_body_text_matches("[a-d]+");
    }

    #[should_panic(expected = "expected text body 'abcd' to match regex '[e-h]+' but did not")]
    #[stubr::mock("body/text/value.json")]
    #[tokio::test]
    async fn expect_body_text_matches_should_fail_when_does_not_match_regex() {
        get(stubr.uri()).await.unwrap().expect_body_text_matches("[e-h]+");
    }

    #[should_panic(expected = "expected a text body but no response body was present")]
    #[stubr::mock("body/text/absent.json")]
    #[tokio::test]
    async fn expect_body_text_should_fail_when_missing() {
        get(stubr.uri()).await.unwrap().expect_body_text_matches("[a-d]+");
    }

    #[tokio::test]
    #[stubr::mock("body/text/value.json")]
    async fn result_should_expect_body_text_matches() {
        get(stubr.uri()).await.expect_body_text_matches("[a-d]+");
    }

    #[should_panic(expected = "expected text body 'abcd' to match regex '[e-h]+' but did not")]
    #[stubr::mock("body/text/value.json")]
    #[tokio::test]
    async fn result_expect_body_text_matches_should_fail_when_does_not_match_regex() {
        get(stubr.uri()).await.expect_body_text_matches("[e-h]+");
    }

    #[should_panic(expected = "expected a text body but no response body was present")]
    #[stubr::mock("body/text/absent.json")]
    #[tokio::test]
    async fn result_expect_body_text_should_fail_when_missing() {
        get(stubr.uri()).await.expect_body_text_matches("[a-d]+");
    }
}

mod bytes {
    use super::*;

    #[tokio::test]
    #[stubr::mock("body/bytes/value.json")]
    async fn should_expect_body_bytes() {
        get(stubr.uri()).await.unwrap().expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    }

    #[should_panic]
    #[stubr::mock("body/bytes/value.json")]
    #[tokio::test]
    async fn expect_body_bytes_should_fail_when_closure_fails() {
        get(stubr.uri()).await.unwrap().expect_body_bytes(|b| assert_eq!(b, b"dcba"));
    }

    #[tokio::test]
    #[stubr::mock("body/bytes/value.json")]
    async fn result_should_expect_body_bytes() {
        get(stubr.uri()).await.expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    }

    #[should_panic]
    #[stubr::mock("body/bytes/value.json")]
    #[tokio::test]
    async fn result_expect_body_bytes_should_fail_when_closure_fails() {
        get(stubr.uri()).await.expect_body_bytes(|b| assert_eq!(b, b"dcba"));
    }
}

mod bytes_eq {
    use super::*;

    #[tokio::test]
    #[stubr::mock("body/bytes/value.json")]
    async fn should_expect_body_bytes_eq() {
        get(stubr.uri()).await.unwrap().expect_body_bytes_eq(b"abcd");
    }

    #[should_panic(expected = "expected body '[97, 98, 99, 100]' to be equal to '[100, 99, 98, 97]' but was not")]
    #[stubr::mock("body/bytes/value.json")]
    #[tokio::test]
    async fn expect_body_bytes_should_fail_not_equal() {
        get(stubr.uri()).await.unwrap().expect_body_bytes_eq(b"dcba");
    }

    #[should_panic(expected = "expected a response body but no response body was present")]
    #[stubr::mock("body/bytes/absent.json")]
    #[tokio::test]
    async fn expect_body_bytes_should_fail_when_missing() {
        get(stubr.uri()).await.unwrap().expect_body_bytes_eq(b"abcd");
    }

    #[tokio::test]
    #[stubr::mock("body/bytes/value.json")]
    async fn result_should_expect_body_bytes_eq() {
        get(stubr.uri()).await.expect_body_bytes_eq(b"abcd");
    }

    #[should_panic(expected = "expected body '[97, 98, 99, 100]' to be equal to '[100, 99, 98, 97]' but was not")]
    #[stubr::mock("body/bytes/value.json")]
    #[tokio::test]
    async fn result_expect_body_bytes_should_fail_not_equal() {
        get(stubr.uri()).await.expect_body_bytes_eq(b"dcba");
    }

    #[should_panic(expected = "expected a response body but no response body was present")]
    #[stubr::mock("body/bytes/absent.json")]
    #[tokio::test]
    async fn result_expect_body_bytes_should_fail_when_missing() {
        get(stubr.uri()).await.expect_body_bytes_eq(b"abcd");
    }
}

mod present {
    use super::*;

    #[tokio::test]
    #[stubr::mock("body/bytes/value.json")]
    async fn should_expect_body_present() {
        get(stubr.uri()).await.unwrap().expect_body_present();
    }

    #[should_panic(expected = "expected a response body but no response body was present")]
    #[stubr::mock("body/bytes/absent.json")]
    #[tokio::test]
    async fn expect_body_present_should_fail_when_absent() {
        get(stubr.uri()).await.unwrap().expect_body_present();
    }

    #[tokio::test]
    #[stubr::mock("body/bytes/value.json")]
    async fn result_should_expect_body_present() {
        get(stubr.uri()).await.expect_body_present();
    }

    #[should_panic(expected = "expected a response body but no response body was present")]
    #[stubr::mock("body/bytes/absent.json")]
    #[tokio::test]
    async fn result_expect_body_present_should_fail_when_absent() {
        get(stubr.uri()).await.expect_body_present();
    }
}

mod absent {
    use super::*;

    #[tokio::test]
    #[stubr::mock("body/bytes/absent.json")]
    async fn should_expect_body_absent() {
        get(stubr.uri()).await.unwrap().expect_body_absent();
    }

    #[should_panic(expected = "expected no response body but a response body was present")]
    #[stubr::mock("body/bytes/value.json")]
    #[tokio::test]
    async fn expect_body_absent_should_fail_when_present() {
        get(stubr.uri()).await.unwrap().expect_body_absent();
    }

    #[tokio::test]
    #[stubr::mock("body/bytes/absent.json")]
    async fn result_should_expect_body_absent() {
        get(stubr.uri()).await.expect_body_absent();
    }

    #[should_panic(expected = "expected no response body but a response body was present")]
    #[stubr::mock("body/bytes/value.json")]
    #[tokio::test]
    async fn result_expect_body_absent_should_fail_when_present() {
        get(stubr.uri()).await.expect_body_absent();
    }
}