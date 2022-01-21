use rocket::{get, http::Header, local::blocking::Client, Request, Response, response::Responder, routes};

use asserhttp::*;

struct Resp(Vec<(&'static str, Vec<&'static str>)>);

impl<'a> Responder<'a, 'static> for Resp {
    fn respond_to(self, _: &'a Request<'_>) -> rocket::response::Result<'static> {
        let mut builder = Response::build();
        for (k, values) in self.0 {
            for v in values { builder.header_adjoin(Header::new(k, v)); }
        }
        builder.ok()
    }
}

mod eq {
    use super::*;

    #[test]
    fn should_expect_header() {
        #[get("/")]
        fn xa() -> Resp { Resp(vec![("x-a", vec!["a"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xa])).unwrap();
        client.get("/").dispatch().expect_header("x-a", "a");
    }

    #[test]
    fn should_expect_header_ignoring_case() {
        #[get("/")]
        fn xa() -> Resp { Resp(vec![("x-a", vec!["a"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xa])).unwrap();
        client.get("/").dispatch().expect_header("X-A", "a");
    }

    #[should_panic(expected = "expected header 'x-a' to be equal to 'A' but was 'a'")]
    #[test]
    fn expect_header_value_should_be_case_sensitive() {
        #[get("/")]
        fn xa() -> Resp { Resp(vec![("x-a", vec!["a"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xa])).unwrap();
        client.get("/").dispatch().expect_header("x-a", "A");
    }

    #[test]
    fn should_expect_many_header() {
        #[get("/")]
        fn xaxb() -> Resp { Resp(vec![("x-a", vec!["a"]), ("x-b", vec!["b"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xaxb])).unwrap();
        client.get("/").dispatch()
            .expect_header("x-a", "a")
            .expect_header("x-b", "b");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[test]
    fn expect_header_should_panic_when_wrong_key() {
        #[get("/")]
        fn xa() -> Resp { Resp(vec![("x-a", vec!["a"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xa])).unwrap();
        client.get("/").dispatch().expect_header("x-b", "a");
    }

    #[should_panic(expected = "expected header 'x-a' to be equal to 'b' but was 'a'")]
    #[test]
    fn expect_header_should_panic_when_wrong_value() {
        #[get("/")]
        fn xa() -> Resp { Resp(vec![("x-a", vec!["a"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xa])).unwrap();
        client.get("/").dispatch().expect_header("x-a", "b");
    }

    #[should_panic(expected = "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.")]
    #[test]
    fn expect_header_should_panic_when_multi_valued() {
        #[get("/")]
        fn xm() -> Resp { Resp(vec![("x-m", vec!["a", "b"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xm])).unwrap();
        client.get("/").dispatch().expect_header("x-m", "a");
    }
}

mod multi {
    use super::*;

    #[test]
    fn expect_headers_should_expect_multi_headers() {
        #[get("/")]
        fn xm() -> Resp { Resp(vec![("x-m", vec!["a", "b"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xm])).unwrap();
        client.get("/").dispatch().expect_headers("x-m", ["a", "b"]);
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[test]
    fn expect_headers_should_fail_when_key_missing() {
        #[get("/")]
        fn xm() -> Resp { Resp(vec![("x-m", vec!["a", "b"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xm])).unwrap();
        client.get("/").dispatch().expect_headers("x-b", ["a", "b"]);
    }

    #[should_panic(expected = "expected header 'x-m' to contain values '[\"a\"]' but contained '[\"a\", \"b\"]'")]
    #[test]
    fn expect_headers_should_fail_when_one_value_missing() {
        #[get("/")]
        fn xm() -> Resp { Resp(vec![("x-m", vec!["a", "b"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xm])).unwrap();
        client.get("/").dispatch().expect_headers("x-m", ["a"]);
    }

    #[should_panic(expected = "expected header 'x-m' to contain values '[\"a\", \"c\"]' but contained '[\"a\", \"b\"]'")]
    #[test]
    fn expect_headers_should_fail_when_one_value_not_eq() {
        #[get("/")]
        fn xm() -> Resp { Resp(vec![("x-m", vec!["a", "b"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xm])).unwrap();
        client.get("/").dispatch().expect_headers("x-m", ["a", "c"]);
    }

    #[should_panic(expected = "expected header 'x-m' to contain values '[\"a\", \"b\", \"c\"]' but contained '[\"a\", \"b\"]'")]
    #[test]
    fn expect_headers_should_fail_when_one_expected_value_missing() {
        #[get("/")]
        fn xm() -> Resp { Resp(vec![("x-m", vec!["a", "b"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xm])).unwrap();
        client.get("/").dispatch().expect_headers("x-m", ["a", "b", "c"]);
    }

    #[should_panic(expected = "no value expected for header 'x-m'. Use 'expect_header_present' instead")]
    #[test]
    fn expect_headers_should_fail_when_no_value_expected() {
        #[get("/")]
        fn xm() -> Resp { Resp(vec![("x-m", vec!["a", "b"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xm])).unwrap();
        client.get("/").dispatch().expect_headers("x-m", []);
    }
}

mod present {
    use super::*;

    #[test]
    fn should_expect_header_present() {
        #[get("/")]
        fn xa() -> Resp { Resp(vec![("x-a", vec!["a"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xa])).unwrap();
        client.get("/").dispatch().expect_header_present("x-a");
    }

    #[test]
    fn should_expect_header_present_ignoring_case() {
        #[get("/")]
        fn xa() -> Resp { Resp(vec![("x-a", vec!["a"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xa])).unwrap();
        client.get("/").dispatch().expect_header_present("X-A");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[test]
    fn expect_header_present_should_fail_when_absent() {
        #[get("/")]
        fn xa() -> Resp { Resp(vec![("x-a", vec!["a"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xa])).unwrap();
        client.get("/").dispatch().expect_header_present("x-b");
    }
}

mod absent {
    use super::*;

    #[test]
    fn should_expect_header_absent() {
        #[get("/")]
        fn xa() -> Resp { Resp(vec![("x-a", vec!["a"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xa])).unwrap();
        client.get("/").dispatch().expect_header_absent("x-b");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[test]
    fn expect_header_absent_should_fail_when_absent() {
        #[get("/")]
        fn xa() -> Resp { Resp(vec![("x-a", vec!["a"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xa])).unwrap();
        client.get("/").dispatch().expect_header_absent("x-a");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[test]
    fn expect_header_absent_should_ignore_case() {
        #[get("/")]
        fn xa() -> Resp { Resp(vec![("x-a", vec!["a"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xa])).unwrap();
        client.get("/").dispatch().expect_header_absent("X-A");
    }
}

mod json {
    use super::*;

    #[test]
    fn should_expect_content_type_json() {
        #[get("/")]
        fn json() -> Resp { Resp(vec![("content-type", vec!["application/json"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![json])).unwrap();
        client.get("/").dispatch().expect_content_type_json();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'")]
    #[test]
    fn expect_header_should_panic_when_wrong_value() {
        #[get("/")]
        fn xml() -> Resp { Resp(vec![("content-type", vec!["application/xml"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xml])).unwrap();
        client.get("/").dispatch().expect_content_type_json();
    }
}

mod text {
    use super::*;

    #[test]
    fn should_expect_content_type_text() {
        #[get("/")]
        fn text() -> Resp { Resp(vec![("content-type", vec!["text/plain"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![text])).unwrap();
        client.get("/").dispatch().expect_content_type_text();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'")]
    #[test]
    fn expect_header_should_panic_when_wrong_value() {
        #[get("/")]
        fn xml() -> Resp { Resp(vec![("content-type", vec!["application/xml"])]) }
        let client = Client::tracked(rocket::build().mount("/", routes![xml])).unwrap();
        client.get("/").dispatch().expect_content_type_text();
    }
}