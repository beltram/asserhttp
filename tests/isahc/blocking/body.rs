use isahc::get;
use serde_json::json;
use stubr::Stubr;

use asserhttp::*;

use super::super::super::common::Body;

mod json {
    use super::*;

    #[test]
    fn should_expect_raw_body_json() {
        let srv = Stubr::start_blocking("tests/stubs/body/value.json");
        get(&srv.uri()).unwrap().expect_body_json(json!({"a": "b"}));
    }

    #[test]
    fn should_expect_struct_body_json() {
        let srv = Stubr::start_blocking("tests/stubs/body/value.json");
        get(&srv.uri()).unwrap().expect_body_json(Body { a: String::from("b") });
    }

    #[test]
    #[should_panic(expected = "expected json body Object({\"a\": String(\"b\")}) to be equal to Object({\"a\": String(\"c\")}) but was not")]
    fn expect_body_json_should_fail_when_not_equal() {
        let srv = Stubr::start_blocking("tests/stubs/body/value.json");
        get(&srv.uri()).unwrap().expect_body_json(json!({"a": "c"}));
    }

    #[test]
    #[should_panic(expected = "expected json body Object({\"a\": String(\"b\")}) but no response body was present")]
    fn expect_body_json_should_fail_when_missing() {
        let srv = Stubr::start_blocking("tests/stubs/body/missing.json");
        get(&srv.uri()).unwrap().expect_body_json(json!({"a": "b"}));
    }

    #[test]
    fn result_should_expect_raw_body_json() {
        let srv = Stubr::start_blocking("tests/stubs/body/value.json");
        get(&srv.uri()).expect_body_json(json!({"a": "b"}));
    }

    #[test]
    fn result_should_expect_struct_body_json() {
        let srv = Stubr::start_blocking("tests/stubs/body/value.json");
        get(&srv.uri()).expect_body_json(Body { a: String::from("b") });
    }

    #[test]
    #[should_panic(expected = "expected json body Object({\"a\": String(\"b\")}) to be equal to Object({\"a\": String(\"c\")}) but was not")]
    fn result_expect_body_json_should_fail_when_not_equal() {
        let srv = Stubr::start_blocking("tests/stubs/body/value.json");
        get(&srv.uri()).expect_body_json(json!({"a": "c"}));
    }

    #[test]
    #[should_panic(expected = "expected json body Object({\"a\": String(\"b\")}) but no response body was present")]
    fn result_expect_body_json_should_fail_when_missing() {
        let srv = Stubr::start_blocking("tests/stubs/body/missing.json");
        get(&srv.uri()).expect_body_json(json!({"a": "b"}));
    }
}