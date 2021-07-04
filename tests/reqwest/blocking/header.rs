use reqwest::blocking::get;

use asserhttp::*;

mod eq {
    use super::*;

    #[test]
    #[stubr::mock("header/one.json")]
    fn should_expect_header() {
        get(stubr.uri()).unwrap().expect_header("x-a", "a");
    }

    #[test]
    #[stubr::mock("header/one.json")]
    fn should_expect_header_ignoring_case() {
        get(stubr.uri()).unwrap().expect_header("X-A", "a");
    }

    #[should_panic(expected = "expected header 'x-a' to be equal to 'A' but was 'a'")]
    #[stubr::mock("header/one.json")]
    #[test]
    fn expect_header_value_should_be_case_sensitive() {
        get(stubr.uri()).unwrap().expect_header("x-a", "A");
    }

    #[test]
    #[stubr::mock("header/many.json")]
    fn should_expect_many_header() {
        get(stubr.uri()).unwrap()
            .expect_header("x-a", "a")
            .expect_header("x-b", "b");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/one.json")]
    #[test]
    fn expect_header_should_panic_when_wrong_key() {
        get(stubr.uri()).unwrap().expect_header("x-b", "a");
    }

    #[should_panic(expected = "expected header 'x-a' to be equal to 'b' but was 'a'")]
    #[stubr::mock("header/one.json")]
    #[test]
    fn expect_header_should_panic_when_wrong_value() {
        get(stubr.uri()).unwrap().expect_header("x-a", "b");
    }

    #[should_panic(expected = "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.")]
    #[stubr::mock("header/multi.json")]
    #[test]
    fn expect_header_should_panic_when_multi_valued() {
        get(stubr.uri()).unwrap().expect_header("x-m", "a");
    }

    #[test]
    #[stubr::mock("header/one.json")]
    fn result_should_expect_header() {
        get(stubr.uri()).expect_header("x-a", "a");
    }

    #[test]
    #[stubr::mock("header/many.json")]
    fn result_should_expect_many_header() {
        get(stubr.uri())
            .expect_header("x-a", "a")
            .expect_header("x-b", "b");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/one.json")]
    #[test]
    fn result_expect_header_should_panic_when_wrong_key() {
        get(stubr.uri()).expect_header("x-b", "a");
    }

    #[should_panic(expected = "expected header 'x-a' to be equal to 'b' but was 'a'")]
    #[stubr::mock("header/one.json")]
    #[test]
    fn result_expect_header_should_panic_when_wrong_value() {
        get(stubr.uri()).expect_header("x-a", "b");
    }

    #[should_panic(expected = "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.")]
    #[stubr::mock("header/multi.json")]
    #[test]
    fn result_expect_header_should_panic_when_multi_valued() {
        get(stubr.uri()).expect_header("x-m", "a");
    }
}

mod multi {
    use super::*;

    #[test]
    #[stubr::mock("header/multi.json")]
    fn expect_headers_should_expect_multi_headers() {
        get(stubr.uri()).unwrap().expect_headers("x-m", ["a", "b"]);
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/multi.json")]
    #[test]
    fn expect_headers_should_fail_when_key_missing() {
        get(stubr.uri()).unwrap().expect_headers("x-b", ["a", "b"]);
    }

    #[should_panic(expected = "expected header 'x-m' to contain values '[\"a\"]' but contained '[\"a\", \"b\"]'")]
    #[stubr::mock("header/multi.json")]
    #[test]
    fn expect_headers_should_fail_when_one_value_missing() {
        get(stubr.uri()).unwrap().expect_headers("x-m", ["a"]);
    }

    #[should_panic(expected = "expected header 'x-m' to contain values '[\"a\", \"c\"]' but contained '[\"a\", \"b\"]'")]
    #[stubr::mock("header/multi.json")]
    #[test]
    fn expect_headers_should_fail_when_one_value_not_eq() {
        get(stubr.uri()).unwrap().expect_headers("x-m", ["a", "c"]);
    }

    #[should_panic(expected = "expected header 'x-m' to contain values '[\"a\", \"b\", \"c\"]' but contained '[\"a\", \"b\"]'")]
    #[stubr::mock("header/multi.json")]
    #[test]
    fn expect_headers_should_fail_when_one_expected_value_missing() {
        get(stubr.uri()).unwrap().expect_headers("x-m", ["a", "b", "c"]);
    }

    #[should_panic(expected = "no value expected for header 'x-m'. Use 'expect_header_present' instead")]
    #[stubr::mock("header/multi.json")]
    #[test]
    fn expect_headers_should_fail_when_no_value_expected() {
        get(stubr.uri()).unwrap().expect_headers("x-m", []);
    }

    #[test]
    #[stubr::mock("header/multi.json")]
    fn result_expect_headers_should_expect_multi_headers() {
        get(stubr.uri()).expect_headers("x-m", ["a", "b"]);
    }
}

mod present {
    use super::*;

    #[test]
    #[stubr::mock("header/one.json")]
    fn should_expect_header_present() {
        get(stubr.uri()).unwrap().expect_header_present("x-a");
    }

    #[test]
    #[stubr::mock("header/one.json")]
    fn should_expect_header_present_ignoring_case() {
        get(stubr.uri()).unwrap().expect_header_present("X-A");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/one.json")]
    #[test]
    fn expect_header_present_should_fail_when_absent() {
        get(stubr.uri()).unwrap().expect_header_present("x-b");
    }

    #[test]
    #[stubr::mock("header/one.json")]
    fn result_should_expect_header_present() {
        get(stubr.uri()).expect_header_present("x-a");
    }

    #[test]
    #[stubr::mock("header/one.json")]
    fn result_should_expect_header_present_ignoring_case() {
        get(stubr.uri()).expect_header_present("X-A");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[stubr::mock("header/one.json")]
    #[test]
    fn result_expect_header_present_should_fail_when_absent() {
        get(stubr.uri()).expect_header_present("x-b");
    }
}

mod absent {
    use super::*;

    #[test]
    #[stubr::mock("header/one.json")]
    fn should_expect_header_absent() {
        get(stubr.uri()).unwrap().expect_header_absent("x-b");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[stubr::mock("header/one.json")]
    #[test]
    fn expect_header_absent_should_fail_when_absent() {
        get(stubr.uri()).unwrap().expect_header_absent("x-a");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[stubr::mock("header/one.json")]
    #[test]
    fn expect_header_absent_should_ignore_case() {
        get(stubr.uri()).unwrap().expect_header_absent("X-A");
    }

    #[test]
    #[stubr::mock("header/one.json")]
    fn result_should_expect_header_absent() {
        get(stubr.uri()).expect_header_absent("x-b");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[stubr::mock("header/one.json")]
    #[test]
    fn result_expect_header_absent_should_fail_when_absent() {
        get(stubr.uri()).expect_header_absent("x-a");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[stubr::mock("header/one.json")]
    #[test]
    fn result_expect_header_absent_should_ignore_case() {
        get(stubr.uri()).expect_header_absent("X-A");
    }
}

mod json {
    use super::*;

    #[test]
    #[stubr::mock("header/json.json")]
    fn should_expect_content_type_json() {
        get(stubr.uri()).unwrap().expect_content_type_json();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'")]
    #[stubr::mock("header/xml.json")]
    #[test]
    fn expect_header_should_panic_when_wrong_value() {
        get(stubr.uri()).unwrap().expect_content_type_json();
    }

    #[test]
    #[stubr::mock("header/json.json")]
    fn result_should_expect_content_type_json() {
        get(stubr.uri()).expect_content_type_json();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'")]
    #[stubr::mock("header/xml.json")]
    #[test]
    fn result_expect_header_should_panic_when_wrong_value() {
        get(stubr.uri()).expect_content_type_json();
    }
}

mod text {
    use super::*;

    #[test]
    #[stubr::mock("header/text.json")]
    fn should_expect_content_type_text() {
        get(stubr.uri()).unwrap().expect_content_type_text();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'")]
    #[stubr::mock("header/xml.json")]
    #[test]
    fn expect_header_should_panic_when_wrong_value() {
        get(stubr.uri()).unwrap().expect_content_type_text();
    }

    #[test]
    #[stubr::mock("header/text.json")]
    fn result_should_expect_content_type_text() {
        get(stubr.uri()).expect_content_type_text();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'")]
    #[stubr::mock("header/xml.json")]
    #[test]
    fn result_expect_header_should_panic_when_wrong_value() {
        get(stubr.uri()).expect_content_type_text();
    }
}