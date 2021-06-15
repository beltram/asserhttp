use isahc::get;
use stubr::Stubr;

use asserhttp::*;

mod eq {
    use super::*;

    #[test]
    fn should_expect_header() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).unwrap().expect_header("x-a", "a");
    }

    #[test]
    fn should_expect_header_ignoring_case() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).unwrap().expect_header("X-A", "a");
    }

    #[test]
    #[should_panic(expected = "expected header 'x-a' to be equal to 'A' but was 'a'")]
    fn expect_header_value_should_be_case_sensitive() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).unwrap().expect_header("x-a", "A");
    }

    #[test]
    fn should_expect_many_header() {
        let srv = Stubr::start_blocking("tests/stubs/header/many.json");
        get(&srv.uri()).unwrap()
            .expect_header("x-a", "a")
            .expect_header("x-b", "b");
    }

    #[test]
    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    fn expect_header_should_panic_when_wrong_key() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).unwrap().expect_header("x-b", "a");
    }

    #[test]
    #[should_panic(expected = "expected header 'x-a' to be equal to 'b' but was 'a'")]
    fn expect_header_should_panic_when_wrong_value() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).unwrap().expect_header("x-a", "b");
    }

    #[test]
    #[should_panic(expected = "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.")]
    fn expect_header_should_panic_when_multi_valued() {
        let srv = Stubr::start_blocking("tests/stubs/header/multi.json");
        get(&srv.uri()).unwrap().expect_header("x-m", "a");
    }

    #[test]
    fn result_should_expect_header() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).expect_header("x-a", "a");
    }

    #[test]
    fn result_should_expect_many_header() {
        let srv = Stubr::start_blocking("tests/stubs/header/many.json");
        get(&srv.uri())
            .expect_header("x-a", "a")
            .expect_header("x-b", "b");
    }

    #[test]
    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    fn result_expect_header_should_panic_when_wrong_key() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).expect_header("x-b", "a");
    }

    #[test]
    #[should_panic(expected = "expected header 'x-a' to be equal to 'b' but was 'a'")]
    fn result_expect_header_should_panic_when_wrong_value() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).expect_header("x-a", "b");
    }

    #[test]
    #[should_panic(expected = "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.")]
    fn result_expect_header_should_panic_when_multi_valued() {
        let srv = Stubr::start_blocking("tests/stubs/header/multi.json");
        get(&srv.uri()).expect_header("x-m", "a");
    }
}

mod present {
    use super::*;

    #[test]
    fn should_expect_header_present() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).unwrap().expect_header_present("x-a");
    }

    #[test]
    fn should_expect_header_present_ignoring_case() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).unwrap().expect_header_present("X-A");
    }

    #[test]
    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    fn expect_header_present_should_fail_when_absent() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).unwrap().expect_header_present("x-b");
    }

    #[test]
    fn result_should_expect_header_present() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).expect_header_present("x-a");
    }

    #[test]
    fn result_should_expect_header_present_ignoring_case() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).expect_header_present("X-A");
    }

    #[test]
    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    fn result_expect_header_present_should_fail_when_absent() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).expect_header_present("x-b");
    }
}

mod absent {
    use super::*;

    #[test]
    fn should_expect_header_absent() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).unwrap().expect_header_absent("x-b");
    }

    #[test]
    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    fn expect_header_absent_should_fail_when_absent() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).unwrap().expect_header_absent("x-a");
    }

    #[test]
    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    fn expect_header_absent_should_ignore_case() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).unwrap().expect_header_absent("X-A");
    }

    #[test]
    fn result_should_expect_header_absent() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).expect_header_absent("x-b");
    }

    #[test]
    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    fn result_expect_header_absent_should_fail_when_absent() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).expect_header_absent("x-a");
    }

    #[test]
    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    fn result_expect_header_absent_should_ignore_case() {
        let srv = Stubr::start_blocking("tests/stubs/header/one.json");
        get(&srv.uri()).expect_header_absent("X-A");
    }
}

mod json {
    use super::*;

    #[test]
    fn should_expect_content_type_json() {
        let srv = Stubr::start_blocking("tests/stubs/header/json.json");
        get(&srv.uri()).unwrap().expect_content_type_json();
    }

    #[test]
    #[should_panic(expected = "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'")]
    fn expect_header_should_panic_when_wrong_value() {
        let srv = Stubr::start_blocking("tests/stubs/header/xml.json");
        get(&srv.uri()).unwrap().expect_content_type_json();
    }

    #[test]
    fn result_should_expect_content_type_json() {
        let srv = Stubr::start_blocking("tests/stubs/header/json.json");
        get(&srv.uri()).expect_content_type_json();
    }

    #[test]
    #[should_panic(expected = "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'")]
    fn result_expect_header_should_panic_when_wrong_value() {
        let srv = Stubr::start_blocking("tests/stubs/header/xml.json");
        get(&srv.uri()).expect_content_type_json();
    }
}

mod text {
    use super::*;

    #[test]
    fn should_expect_content_type_text() {
        let srv = Stubr::start_blocking("tests/stubs/header/text.json");
        get(&srv.uri()).unwrap().expect_content_type_text();
    }

    #[test]
    #[should_panic(expected = "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'")]
    fn expect_header_should_panic_when_wrong_value() {
        let srv = Stubr::start_blocking("tests/stubs/header/xml.json");
        get(&srv.uri()).unwrap().expect_content_type_text();
    }

    #[test]
    fn result_should_expect_content_type_text() {
        let srv = Stubr::start_blocking("tests/stubs/header/text.json");
        get(&srv.uri()).expect_content_type_text();
    }

    #[test]
    #[should_panic(expected = "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'")]
    fn result_expect_header_should_panic_when_wrong_value() {
        let srv = Stubr::start_blocking("tests/stubs/header/xml.json");
        get(&srv.uri()).expect_content_type_text();
    }
}