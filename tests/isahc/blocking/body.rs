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
        let srv = Stubr::start_blocking("tests/stubs/body/json/absent.json");
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
        let srv = Stubr::start_blocking("tests/stubs/body/json/absent.json");
        get(&srv.uri()).expect_body_json_eq(json!({"a": "b"}));
    }
}

mod text {
    use super::*;

    #[test]
    fn should_expect_body_text() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).unwrap().expect_body_text(|b| assert_eq!(b, String::from("abcd")));
    }

    #[test]
    #[should_panic]
    fn expect_body_text_should_fail_when_closure_fails() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).unwrap().expect_body_text(|b| assert_eq!(b, String::from("dcba")));
    }

    #[test]
    fn result_should_expect_body_text() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).expect_body_text(|b| assert_eq!(b, String::from("abcd")));
    }

    #[test]
    #[should_panic]
    fn result_expect_body_text_should_fail_when_closure_fails() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).expect_body_text(|b| assert_eq!(b, String::from("dcba")));
    }
}

mod text_eq {
    use super::*;

    #[test]
    fn should_expect_body_text_eq() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).unwrap().expect_body_text_eq("abcd");
    }

    #[test]
    #[should_panic(expected = "expected text body 'abcd' to be equal to 'dcab' but was not")]
    fn expect_body_text_should_fail_when_not_equal() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).unwrap().expect_body_text_eq("dcab");
    }

    #[test]
    #[should_panic(expected = "expected a text body but no response body was present")]
    fn expect_body_text_should_fail_when_missing() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/absent.json");
        get(&srv.uri()).unwrap().expect_body_text_eq("abcd");
    }

    #[test]
    fn result_should_expect_body_text_eq() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).expect_body_text_eq("abcd");
    }

    #[test]
    #[should_panic(expected = "expected text body 'abcd' to be equal to 'dcab' but was not")]
    fn result_expect_body_text_should_fail_when_not_equal() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).expect_body_text_eq("dcab");
    }

    #[test]
    #[should_panic(expected = "expected a text body but no response body was present")]
    fn result_expect_body_text_should_fail_when_missing() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/absent.json");
        get(&srv.uri()).expect_body_text_eq("abcd");
    }
}

mod regex {
    use super::*;

    #[test]
    fn should_expect_body_text_matches() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).unwrap().expect_body_text_matches("[a-d]+");
    }

    #[test]
    #[should_panic(expected = "expected text body 'abcd' to match regex '[e-h]+' but did not")]
    fn expect_body_text_matches_should_fail_when_does_not_match_regex() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).unwrap().expect_body_text_matches("[e-h]+");
    }

    #[test]
    #[should_panic(expected = "expected a text body but no response body was present")]
    fn expect_body_text_should_fail_when_missing() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/absent.json");
        get(&srv.uri()).unwrap().expect_body_text_matches("[a-d]+");
    }

    #[test]
    fn result_should_expect_body_text_matches() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).expect_body_text_matches("[a-d]+");
    }

    #[test]
    #[should_panic(expected = "expected text body 'abcd' to match regex '[e-h]+' but did not")]
    fn result_expect_body_text_matches_should_fail_when_does_not_match_regex() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/value.json");
        get(&srv.uri()).expect_body_text_matches("[e-h]+");
    }

    #[test]
    #[should_panic(expected = "expected a text body but no response body was present")]
    fn result_expect_body_text_should_fail_when_missing() {
        let srv = Stubr::start_blocking("tests/stubs/body/text/absent.json");
        get(&srv.uri()).expect_body_text_matches("[a-d]+");
    }
}

mod bytes {
    use super::*;

    #[test]
    fn should_expect_body_bytes() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/value.json");
        get(&srv.uri()).unwrap().expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    }

    #[test]
    #[should_panic]
    fn expect_body_bytes_should_fail_when_closure_fails() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/value.json");
        get(&srv.uri()).unwrap().expect_body_bytes(|b| assert_eq!(b, b"dcba"));
    }

    #[test]
    fn result_should_expect_body_bytes() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/value.json");
        get(&srv.uri()).expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    }

    #[test]
    #[should_panic]
    fn result_expect_body_bytes_should_fail_when_closure_fails() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/value.json");
        get(&srv.uri()).expect_body_bytes(|b| assert_eq!(b, b"dcba"));
    }
}

mod bytes_eq {
    use super::*;

    #[test]
    fn should_expect_body_bytes_eq() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/value.json");
        get(&srv.uri()).unwrap().expect_body_bytes_eq(b"abcd");
    }

    #[test]
    #[should_panic(expected = "expected body '[97, 98, 99, 100]' to be equal to '[100, 99, 98, 97]' but was not")]
    fn expect_body_bytes_should_fail_not_equal() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/value.json");
        get(&srv.uri()).unwrap().expect_body_bytes_eq(b"dcba");
    }

    #[test]
    #[should_panic(expected = "expected a response body but no response body was present")]
    fn expect_body_bytes_should_fail_when_missing() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/absent.json");
        get(&srv.uri()).unwrap().expect_body_bytes_eq(b"abcd");
    }

    #[test]
    fn result_should_expect_body_bytes_eq() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/value.json");
        get(&srv.uri()).expect_body_bytes_eq(b"abcd");
    }

    #[test]
    #[should_panic(expected = "expected body '[97, 98, 99, 100]' to be equal to '[100, 99, 98, 97]' but was not")]
    fn result_expect_body_bytes_should_fail_not_equal() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/value.json");
        get(&srv.uri()).expect_body_bytes_eq(b"dcba");
    }

    #[test]
    #[should_panic(expected = "expected a response body but no response body was present")]
    fn result_expect_body_bytes_should_fail_when_missing() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/absent.json");
        get(&srv.uri()).expect_body_bytes_eq(b"abcd");
    }
}

mod present {
    use super::*;

    #[test]
    fn should_expect_body_present() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/value.json");
        get(&srv.uri()).unwrap().expect_body_present();
    }

    #[test]
    #[should_panic(expected = "expected a response body but no response body was present")]
    fn expect_body_present_should_fail_when_absent() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/absent.json");
        get(&srv.uri()).unwrap().expect_body_present();
    }

    #[test]
    fn result_should_expect_body_present() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/value.json");
        get(&srv.uri()).expect_body_present();
    }

    #[test]
    #[should_panic(expected = "expected a response body but no response body was present")]
    fn result_expect_body_present_should_fail_when_absent() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/absent.json");
        get(&srv.uri()).expect_body_present();
    }
}

mod absent {
    use super::*;

    #[test]
    fn should_expect_body_absent() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/absent.json");
        get(&srv.uri()).unwrap().expect_body_absent();
    }

    #[test]
    #[should_panic(expected = "expected no response body but a response body was present")]
    fn expect_body_absent_should_fail_when_present() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/value.json");
        get(&srv.uri()).unwrap().expect_body_absent();
    }

    #[test]
    fn result_should_expect_body_absent() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/absent.json");
        get(&srv.uri()).expect_body_absent();
    }

    #[test]
    #[should_panic(expected = "expected no response body but a response body was present")]
    fn result_expect_body_absent_should_fail_when_present() {
        let srv = Stubr::start_blocking("tests/stubs/body/bytes/value.json");
        get(&srv.uri()).expect_body_absent();
    }
}