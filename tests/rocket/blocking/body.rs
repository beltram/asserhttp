use rocket::{get, http::Status, local::blocking::Client, routes};
use serde_json::{json, Value};

use asserhttp::*;

use super::super::super::common::Body;

mod json {
    use super::*;

    #[test]
    fn should_expect_raw_body_json() {
        #[get("/")]
        fn ab() -> Value { json!({"a": "b"}) }
        let client = Client::tracked(rocket::build().mount("/", routes![ab])).unwrap();
        client.get("/").dispatch().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"})));
    }

    #[test]
    fn should_expect_struct_body_json() {
        #[get("/")]
        fn ab() -> Value { json!({"a": "b"}) }
        let client = Client::tracked(rocket::build().mount("/", routes![ab])).unwrap();
        client.get("/").dispatch().expect_body_json(|b: Body| assert_eq!(b, Body { a: String::from("b") }));
    }

    #[should_panic]
    #[test]
    fn expect_body_json_should_fail_when_closure_fails() {
        #[get("/")]
        fn ab() -> Value { json!({"a": "b"}) }
        let client = Client::tracked(rocket::build().mount("/", routes![ab])).unwrap();
        client.get("/").dispatch().expect_body_json(|b: Value| assert_eq!(b, json!({"a": "c"})));
    }
}

mod json_eq {
    use super::*;

    #[test]
    fn should_expect_raw_body_json() {
        #[get("/")]
        fn ab() -> Value { json!({"a": "b"}) }
        let client = Client::tracked(rocket::build().mount("/", routes![ab])).unwrap();
        client.get("/").dispatch().expect_body_json_eq(json!({"a": "b"}));
    }

    #[test]
    fn should_expect_struct_body_json() {
        #[get("/")]
        fn ab() -> Value { json!({"a": "b"}) }
        let client = Client::tracked(rocket::build().mount("/", routes![ab])).unwrap();
        client.get("/").dispatch().expect_body_json_eq(Body { a: String::from("b") });
    }

    #[should_panic(expected = "expected json body '{\"a\":\"b\"}' to be equal to '{\"a\":\"c\"}' but was not")]
    #[test]
    fn expect_body_json_should_fail_when_not_equal() {
        #[get("/")]
        fn ab() -> Value { json!({"a": "b"}) }
        let client = Client::tracked(rocket::build().mount("/", routes![ab])).unwrap();
        client.get("/").dispatch().expect_body_json_eq(json!({"a": "c"}));
    }

    #[should_panic(expected = "expected a json body but no response body was present")]
    #[test]
    fn expect_body_json_should_fail_when_missing() {
        #[get("/")]
        fn ok() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![ok])).unwrap();
        client.get("/").dispatch().expect_body_json_eq(json!({"a": "b"}));
    }
}

mod text {
    use super::*;

    #[test]
    fn should_expect_body_text() {
        #[get("/")]
        fn abcd() -> &'static str { "abcd" }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_text(|b| assert_eq!(b, String::from("abcd")));
    }

    #[should_panic]
    #[test]
    fn expect_body_text_should_fail_when_closure_fails() {
        #[get("/")]
        fn abcd() -> &'static str { "abcd" }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_text(|b| assert_eq!(b, String::from("dcba")));
    }
}

mod text_eq {
    use super::*;

    #[test]
    fn should_expect_body_text_eq() {
        #[get("/")]
        fn abcd() -> &'static str { "abcd" }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_text_eq("abcd");
    }

    #[should_panic(expected = "expected text body 'abcd' to be equal to 'dcab' but was not")]
    #[test]
    fn expect_body_text_should_fail_when_not_equal() {
        #[get("/")]
        fn abcd() -> &'static str { "abcd" }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_text_eq("dcab");
    }

    #[should_panic(expected = "expected a text body but no response body was present")]
    #[test]
    fn expect_body_text_should_fail_when_missing() {
        #[get("/")]
        fn abcd() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_text_eq("abcd");
    }
}

mod regex {
    use super::*;

    #[test]
    fn should_expect_body_text_matches() {
        #[get("/")]
        fn abcd() -> &'static str { "abcd" }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_text_matches("[a-d]+");
    }

    #[should_panic(expected = "expected text body 'abcd' to match regex '[e-h]+' but did not")]
    #[test]
    fn expect_body_text_matches_should_fail_when_does_not_match_regex() {
        #[get("/")]
        fn abcd() -> &'static str { "abcd" }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_text_matches("[e-h]+");
    }

    #[should_panic(expected = "expected a text body but no response body was present")]
    #[test]
    fn expect_body_text_should_fail_when_missing() {
        #[get("/")]
        fn abcd() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_text_matches("[a-d]+");
    }
}

mod bytes {
    use super::*;

    #[test]
    fn should_expect_body_bytes() {
        #[get("/")]
        fn abcd() -> &'static str { "abcd" }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_bytes(|b| assert_eq!(b, b"abcd"));
    }

    #[should_panic]
    #[test]
    fn expect_body_bytes_should_fail_when_closure_fails() {
        #[get("/")]
        fn abcd() -> &'static str { "abcd" }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_bytes(|b| assert_eq!(b, b"dcba"));
    }
}

mod bytes_eq {
    use super::*;

    #[test]
    fn should_expect_body_bytes_eq() {
        #[get("/")]
        fn abcd() -> &'static str { "abcd" }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_bytes_eq(b"abcd");
    }

    #[should_panic(expected = "expected body '[97, 98, 99, 100]' to be equal to '[100, 99, 98, 97]' but was not")]
    #[test]
    fn expect_body_bytes_should_fail_not_equal() {
        #[get("/")]
        fn abcd() -> &'static str { "abcd" }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_bytes_eq(b"dcba");
    }

    #[should_panic(expected = "expected a response body but no response body was present")]
    #[test]
    fn expect_body_bytes_should_fail_when_missing() {
        #[get("/")]
        fn abcd() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_bytes_eq(b"abcd");
    }
}

mod present {
    use super::*;

    #[test]
    fn should_expect_body_present() {
        #[get("/")]
        fn abcd() -> &'static str { "abcd" }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_present();
    }

    #[should_panic(expected = "expected a response body but no response body was present")]
    #[test]
    fn expect_body_present_should_fail_when_absent() {
        #[get("/")]
        fn abcd() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_present();
    }
}

mod absent {
    use super::*;

    #[test]
    fn should_expect_body_absent() {
        #[get("/")]
        fn abcd() -> Status { Status::Ok }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_absent();
    }

    #[should_panic(expected = "expected no response body but a response body was present")]
    #[test]
    fn expect_body_absent_should_fail_when_present() {
        #[get("/")]
        fn abcd() -> &'static str { "abcd" }
        let client = Client::tracked(rocket::build().mount("/", routes![abcd])).unwrap();
        client.get("/").dispatch().expect_body_absent();
    }
}