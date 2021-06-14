use isahc::get;
use serde_json::{json, Value};
use stubr::Stubr;

use asserhttp::*;

use super::super::super::common::Body;

mod json {
    use super::*;

    #[test]
    fn should_expect_raw_body_json() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/value.json");
        get(&srv.uri()).unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    }

    #[test]
    fn should_expect_struct_body_json() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/value.json");
        get(&srv.uri()).unwrap().expect_body_json(|b: Body| assert_eq!(b, Body { a: String::from("b") }));
    }

    #[test]
    #[should_panic]
    fn expect_body_json_should_fail_when_closure_fails() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/value.json");
        get(&srv.uri()).unwrap().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "c"})));
    }

    #[test]
    fn result_should_expect_raw_body_json() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/value.json");
        get(&srv.uri()).expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    }

    #[test]
    fn result_should_expect_struct_body_json() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/value.json");
        get(&srv.uri()).expect_body_json(|b: Body| assert_eq!(b, Body { a: String::from("b") }));
    }

    #[test]
    #[should_panic]
    fn result_expect_body_json_should_fail_when_closure_fails() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/value.json");
        get(&srv.uri()).expect_body_json(|b: Value| assert_eq!(b, json!({"a": "c"})));
    }
}

mod json_eq {
    use super::*;

    #[test]
    fn should_expect_raw_body_json() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/value.json");
        get(&srv.uri()).unwrap().expect_body_json_eq(json!({"a": "b"}));
    }

    #[test]
    fn should_expect_struct_body_json() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/value.json");
        get(&srv.uri()).unwrap().expect_body_json_eq(Body { a: String::from("b") });
    }

    #[test]
    #[should_panic(expected = "expected json body Object({\"a\": String(\"b\")}) to be equal to Object({\"a\": String(\"c\")}) but was not")]
    fn expect_body_json_should_fail_when_not_equal() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/value.json");
        get(&srv.uri()).unwrap().expect_body_json_eq(json!({"a": "c"}));
    }

    #[test]
    #[should_panic(expected = "expected a json body but no response body was present")]
    fn expect_body_json_should_fail_when_missing() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/missing.json");
        get(&srv.uri()).unwrap().expect_body_json_eq(json!({"a": "b"}));
    }

    #[test]
    fn result_should_expect_raw_body_json() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/value.json");
        get(&srv.uri()).expect_body_json_eq(json!({"a": "b"}));
    }

    #[test]
    fn result_should_expect_struct_body_json() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/value.json");
        get(&srv.uri()).expect_body_json_eq(Body { a: String::from("b") });
    }

    #[test]
    #[should_panic(expected = "expected json body Object({\"a\": String(\"b\")}) to be equal to Object({\"a\": String(\"c\")}) but was not")]
    fn result_expect_body_json_should_fail_when_not_equal() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/value.json");
        get(&srv.uri()).expect_body_json_eq(json!({"a": "c"}));
    }

    #[test]
    #[should_panic(expected = "expected a json body but no response body was present")]
    fn result_expect_body_json_should_fail_when_missing() {
        let srv = Stubr::start_blocking("tests/stubs/body/json/missing.json");
        get(&srv.uri()).expect_body_json_eq(json!({"a": "b"}));
    }
}

mod text {
    use super::*;

    #[test]
    fn should_expect_body_text() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).unwrap().expect_body_text("abcd");
    }

    #[test]
    #[should_panic(expected = "expected text body 'abcd' to be equal to 'dcab' but was not")]
    fn expect_body_text_should_fail_when_not_equal() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).unwrap().expect_body_text("dcab");
    }

    #[test]
    #[should_panic(expected = "expected text body 'abcd' but no response body was present")]
    fn expect_body_text_should_fail_when_missing() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/missing.json");
        get(&srv.uri()).unwrap().expect_body_text("abcd");
    }

    #[test]
    fn result_should_expect_body_text() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).expect_body_text("abcd");
    }

    #[test]
    #[should_panic(expected = "expected text body 'abcd' to be equal to 'dcab' but was not")]
    fn result_expect_body_text_should_fail_when_not_equal() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).expect_body_text("dcab");
    }

    #[test]
    #[should_panic(expected = "expected text body 'abcd' but no response body was present")]
    fn result_expect_body_text_should_fail_when_missing() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/missing.json");
        get(&srv.uri()).expect_body_text("abcd");
    }
}