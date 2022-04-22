use std::io::Cursor;

use rocket::{http::Header, Request, response, Response, response::Responder};
use serde_json::Value;

use utils::TestBody;

#[path = "./utils.rs"]
mod utils;

struct Resp(Response<'static>);

impl Responder<'_, 'static> for Resp {
    fn respond_to(self, _: &'_ Request<'_>) -> response::Result<'static> {
        Ok(self.0)
    }
}

impl From<rocket::http::Status> for Resp {
    fn from(status: rocket::http::Status) -> Self {
        Self(Response::build().status(status).finalize())
    }
}

impl From<Vec<(&'static str, Vec<&'static str>)>> for Resp {
    fn from(headers: Vec<(&'static str, Vec<&'static str>)>) -> Self {
        let mut builder = Response::build();
        for (key, values) in headers {
            for v in values {
                builder.header_adjoin(Header::new(key, v));
            }
        }
        Self(builder.finalize())
    }
}

impl From<Vec<(&'static str, &'static str)>> for Resp {
    fn from(headers: Vec<(&'static str, &'static str)>) -> Self {
        Self::from(headers.into_iter().map(|(k, v)| (k, vec![v])).collect::<Vec<_>>())
    }
}

impl From<Value> for Resp {
    fn from(json: Value) -> Self {
        Self(Response::build().streamed_body(Cursor::new(json.to_string())).finalize())
    }
}

impl From<&'static str> for Resp {
    fn from(body: &'static str) -> Self {
        Self(Response::build().streamed_body(Cursor::new(body)).finalize())
    }
}

// For rocket
#[macro_export]
macro_rules! rocket_test {
    ($fn_name:ident, $init:expr, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // blocking
            #[test]
            fn [<rocket_blocking_ $fn_name>]() {
                use asserhttp::*;
                #[rocket::get("/")]
                fn endpoint() -> crate::Resp { crate::Resp::from($init) }
                let client = rocket::local::blocking::Client::tracked(rocket::build().mount("/", rocket::routes![endpoint])).unwrap();
                $(client.get("/").dispatch()$( .$meth($($arg),*) )+;)+
            }

            // async
            #[rocket::async_test]
            async fn [<rocket_async_ $fn_name>]() {
                use asserhttp::*;
                #[rocket::get("/")]
                fn endpoint() -> crate::Resp { crate::Resp::from($init) }
                let client = rocket::local::asynchronous::Client::tracked(rocket::build().mount("/", rocket::routes![endpoint])).await.unwrap();
                $(client.get("/").dispatch().await$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $init:expr, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // blocking
            #[should_panic(expected = $panic_msg)]
            #[test]
            fn [<rocket_blocking_ $fn_name>]() {
                use asserhttp::*;
                #[rocket::get("/")]
                fn endpoint() -> crate::Resp { crate::Resp::from($init) }
                let client = rocket::local::blocking::Client::tracked(rocket::build().mount("/", rocket::routes![endpoint])).unwrap();
                $(client.get("/").dispatch()$( .$meth($($arg),*) )+;)+
            }

            // async
            #[should_panic(expected = $panic_msg)]
            #[rocket::async_test]
            async fn [<rocket_async_ $fn_name>]() {
                use asserhttp::*;
                #[rocket::get("/")]
                fn endpoint() -> crate::Resp { crate::Resp::from($init) }
                let client = rocket::local::asynchronous::Client::tracked(rocket::build().mount("/", rocket::routes![endpoint])).await.unwrap();
                $(client.get("/").dispatch().await$( .$meth($($arg),*) )+;)+
            }
        }
    };
}

mod smoke {
    rocket_test!(simple_should_succeed, rocket::http::Status::Ok, .expect_status_eq(200));
    rocket_test!(simple_should_fail, rocket::http::Status::Ok, "", .expect_status_eq(100));
    rocket_test!(simple_should_chain, rocket::http::Status::Ok, .expect_status_eq(200).expect_status_eq(200));
}

mod status {
    rocket_test!(status_eq_should_succeed, rocket::http::Status::Ok, .expect_status_eq(200));
    rocket_test!(status_eq_enum_should_succeed, rocket::http::Status::Ok, .expect_status_eq(Status::Ok));
    rocket_test!(status_eq_should_fail, rocket::http::Status::Ok, "expected status to be '100' but was '200'", .expect_status_eq(100));

    rocket_test!(status_ok_should_succeed, rocket::http::Status::Ok, .expect_status_ok());
    rocket_test!(status_ok_should_fail, rocket::http::Status::Created, "expected status to be '200' but was '201'", .expect_status_ok());
    rocket_test!(status_created_should_succeed, rocket::http::Status::Created, .expect_status_created());
    rocket_test!(status_created_should_fail, rocket::http::Status::Ok, "expected status to be '201' but was '200'", .expect_status_created());
    rocket_test!(status_accepted_should_succeed, rocket::http::Status::Accepted, .expect_status_accepted());
    rocket_test!(status_accepted_should_fail, rocket::http::Status::Ok, "expected status to be '202' but was '200'", .expect_status_accepted());
    rocket_test!(status_no_content_should_succeed, rocket::http::Status::NoContent, .expect_status_no_content());
    rocket_test!(status_no_content_should_fail, rocket::http::Status::Ok, "expected status to be '204' but was '200'", .expect_status_no_content());
    rocket_test!(status_partial_content_should_succeed, rocket::http::Status::PartialContent, .expect_status_partial_content());
    rocket_test!(status_partial_content_should_fail, rocket::http::Status::Ok, "expected status to be '206' but was '200'", .expect_status_partial_content());
    rocket_test!(status_bad_request_should_succeed, rocket::http::Status::BadRequest, .expect_status_bad_request());
    rocket_test!(status_bad_request_should_fail, rocket::http::Status::Ok, "expected status to be '400' but was '200'", .expect_status_bad_request());
    rocket_test!(status_unauthorized_should_succeed, rocket::http::Status::Unauthorized, .expect_status_unauthorized());
    rocket_test!(status_unauthorized_should_fail, rocket::http::Status::Ok, "expected status to be '401' but was '200'", .expect_status_unauthorized());
    rocket_test!(status_forbidden_should_succeed, rocket::http::Status::Forbidden, .expect_status_forbidden());
    rocket_test!(status_forbidden_should_fail, rocket::http::Status::Ok, "expected status to be '403' but was '200'", .expect_status_forbidden());
    rocket_test!(status_not_found_should_succeed, rocket::http::Status::NotFound, .expect_status_not_found());
    rocket_test!(status_not_found_should_fail, rocket::http::Status::Ok, "expected status to be '404' but was '200'", .expect_status_not_found());
    rocket_test!(status_conflict_should_succeed, rocket::http::Status::Conflict, .expect_status_conflict());
    rocket_test!(status_conflict_should_fail, rocket::http::Status::Ok, "expected status to be '409' but was '200'", .expect_status_conflict());
    rocket_test!(status_gone_should_succeed, rocket::http::Status::Gone, .expect_status_gone());
    rocket_test!(status_gone_should_fail, rocket::http::Status::Ok, "expected status to be '410' but was '200'", .expect_status_gone());
    rocket_test!(status_internal_server_error_should_succeed, rocket::http::Status::InternalServerError, .expect_status_internal_server_error());
    rocket_test!(status_internal_server_error_should_fail, rocket::http::Status::Ok, "expected status to be '500' but was '200'", .expect_status_internal_server_error());

    rocket_test!(status_range_should_succeed, rocket::http::Status::Created, .expect_status_in_range(200, 300));
    rocket_test!(status_range_should_fail_when_higher, rocket::http::Status::InternalServerError, "expected status to be in [300;400[ but was '500'", .expect_status_in_range(300, 400));
    rocket_test!(status_range_should_fail_when_lower, rocket::http::Status::Ok, "expected status to be in [300;400[ but was '200'", .expect_status_in_range(300, 400));
    rocket_test!(status_range_should_succeed_in_inclusive_lower_range, rocket::http::Status::Ok, .expect_status_in_range(200, 202));
    rocket_test!(status_range_should_fail_when_in_exclusive_upper_range, rocket::http::Status::Accepted, "expected status to be in [200;202[ but was '202'", .expect_status_in_range(200, 202));

    rocket_test!(status_success_should_succeed, rocket::http::Status::Ok, .expect_status_success());
    rocket_test!(status_success_should_fail, rocket::http::Status::BadRequest, "expected status to be in [200;300[ but was '400'", .expect_status_success());
    rocket_test!(status_redirection_should_succeed, rocket::http::Status::MovedPermanently, .expect_status_redirection());
    rocket_test!(status_redirection_should_fail, rocket::http::Status::InternalServerError, "expected status to be in [300;400[ but was '500'", .expect_status_redirection());
    rocket_test!(status_client_error_should_succeed, rocket::http::Status::BadRequest, .expect_status_client_error());
    rocket_test!(status_client_error_should_fail, rocket::http::Status::Ok, "expected status to be in [400;500[ but was '200'", .expect_status_client_error());
    rocket_test!(status_server_error_should_succeed, rocket::http::Status::InternalServerError, .expect_status_server_error());
    rocket_test!(status_server_error_should_fail, rocket::http::Status::Ok, "expected status to be in [500;600[ but was '200'", .expect_status_server_error());
}

mod header {
    rocket_test!(header_eq_should_succeed, vec![("x-a", "a")], .expect_header("x-a", "a"));
    rocket_test!(header_eq_should_match_key_ignoring_case, vec![("x-a", "a")], .expect_header("X-A", "a"));
    rocket_test!(header_eq_should_fail_because_value_case_sensitive, vec![("x-a", "a")], "expected header 'x-a' to be equal to 'A' but was 'a'", .expect_header("x-a", "A"));
    rocket_test!(header_eq_should_fail_when_wrong_key, vec![("x-a", "a")], "expected one header named 'x-b' but none found", .expect_header("x-b", "a"));
    rocket_test!(header_eq_should_fail_when_wrong_value, vec![("x-a", "a")], "expected header 'x-a' to be equal to 'b' but was 'a'", .expect_header("x-a", "b"));
    rocket_test!(header_eq_many_should_succeed, vec![("x-a", "a"), ("x-b", "b")], .expect_header("x-a", "a").expect_header("x-b", "b"));
    rocket_test!(header_eq_should_fail_when_multivalued, vec![("x-m", vec!["a", "b"])], "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.", .expect_header("x-m", "a"));

    rocket_test!(header_multi_should_succeed, vec![("x-m", vec!["a", "b"])], .expect_headers("x-m", ["a", "b"]));
    rocket_test!(header_multi_should_fail_when_key_missing, vec![("x-m", vec!["a", "b"])], "expected one header named 'x-b' but none found", .expect_headers("x-b", ["a", "b"]));
    rocket_test!(header_multi_should_fail_when_one_value_missing, vec![("x-m", vec!["a", "b"])], "expected header 'x-m' to contain values '[\"a\"]' but contained '[\"a\", \"b\"]'", .expect_headers("x-m", ["a"]));
    rocket_test!(header_multi_should_fail_when_one_value_not_eq, vec![("x-m", vec!["a", "b"])], "expected header 'x-m' to contain values '[\"a\", \"c\"]' but contained '[\"a\", \"b\"]'", .expect_headers("x-m", ["a", "c"]));
    rocket_test!(header_multi_should_fail_when_one_expected_value_missing, vec![("x-m", vec!["a", "b"])], "expected header 'x-m' to contain values '[\"a\", \"b\", \"c\"]' but contained '[\"a\", \"b\"]'", .expect_headers("x-m", ["a", "b", "c"]));
    rocket_test!(header_multi_should_fail_when_no_value_expected, vec![("x-m", vec!["a", "b"])], "no value expected for header 'x-m'. Use 'expect_header_present' instead", .expect_headers("x-m", []));

    rocket_test!(header_present_should_succeed, vec![("x-a", "a")], .expect_header_present("x-a"));
    rocket_test!(header_present_should_succeed_ignoring_case, vec![("x-a", "a")], .expect_header_present("X-A"));
    rocket_test!(header_present_should_fail_when_absent, vec![("x-a", "a")], "expected one header named 'x-b' but none found", .expect_header_present("x-b"));

    rocket_test!(header_absent_should_succeed, vec![("x-a", "a")], .expect_header_absent("x-b"));
    rocket_test!(header_absent_should_fail_when_present, vec![("x-a", "a")], "expected no header named 'x-a' but one found", .expect_header_absent("x-a"));
    rocket_test!(header_absent_should_fail_when_present_ignoring_case, vec![("x-a", "a")], "expected no header named 'x-a' but one found", .expect_header_absent("X-A"));

    rocket_test!(header_content_type_json_should_succeed, vec![("content-type", "application/json")], .expect_content_type_json());
    rocket_test!(header_content_type_json_should_fail, vec![("content-type", "application/xml")], "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'", .expect_content_type_json());

    rocket_test!(header_content_type_text_should_succeed, vec![("content-type", "text/plain")], .expect_content_type_text());
    rocket_test!(header_content_type_text_should_fail, vec![("content-type", "application/xml")], "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'", .expect_content_type_text());
}

mod body {
    use serde_json::{json, Value};

    use super::*;

    rocket_test!(body_json_should_succeed, json!({"a": "b"}), .expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"}))));
    rocket_test!(body_json_struct_should_succeed, json!({"a": "b"}), .expect_body_json(|b: TestBody| assert_eq!(b, TestBody { a: String::from("b") })));
    rocket_test!(body_json_should_fail_when_closure_fails, json!({"a": "b"}), "", .expect_body_json(|b: Value| assert_eq!(b, json!({"a": "c"}))));

    rocket_test!(body_json_eq_should_succeed, json!({"a": "b"}), .expect_body_json_eq(json!({"a": "b"})));
    rocket_test!(body_json_struct_eq_should_succeed, json!({"a": "b"}), .expect_body_json_eq(TestBody { a: String::from("b") }));
    rocket_test!(body_json_eq_should_fail_when_not_eq, json!({"a": "b"}), "expected json body '{\"a\":\"b\"}' to be equal to '{\"a\":\"c\"}' but was not", .expect_body_json_eq(json!({"a": "c"})));
    rocket_test!(body_json_eq_should_fail_when_absent, rocket::http::Status::Ok, "expected a json body but no response body was present", .expect_body_json_eq(json!({"a": "b"})));

    rocket_test!(body_text_should_succeed, "abcd", .expect_body_text(|b| assert_eq!(b, String::from("abcd"))));
    rocket_test!(body_text_should_fail_when_closure_fails, "abcd", "", .expect_body_text(|b| assert_eq!(b, String::from("dcba"))));

    rocket_test!(body_text_eq_should_succeed, "abcd", .expect_body_text_eq("abcd"));
    rocket_test!(body_text_eq_should_fail_when_not_eq, "abcd", "expected text body 'abcd' to be equal to 'dcba' but was not", .expect_body_text_eq("dcba"));
    rocket_test!(body_text_eq_should_fail_when_absent, rocket::http::Status::Ok, "expected a text body but no response body was present", .expect_body_text_eq("abcd"));

    rocket_test!(body_text_regex_should_succeed, "abcd", .expect_body_text_matches("[a-d]+"));
    rocket_test!(body_text_regex_should_fail_when_does_not_match, "abcd", "expected text body 'abcd' to match regex '[e-h]+' but did not", .expect_body_text_matches("[e-h]+"));
    rocket_test!(body_text_regex_should_fail_when_absent, rocket::http::Status::Ok, "expected a text body but no response body was present", .expect_body_text_matches("[a-d]+"));

    rocket_test!(body_bytes_should_succeed, "abcd", .expect_body_bytes(|b| assert_eq!(b, b"abcd")));
    rocket_test!(body_bytes_should_fail_when_closure_fails, "abcd", "", .expect_body_bytes(|b| assert_eq!(b, b"dcba")));

    rocket_test!(body_bytes_eq_should_succeed, "abcd", .expect_body_bytes_eq(b"abcd"));
    rocket_test!(body_bytes_eq_should_fail_when_not_eq, "abcd", "expected body '[97, 98, 99, 100]' to be equal to '[100, 99, 98, 97]' but was not", .expect_body_bytes_eq(b"dcba"));
    rocket_test!(body_bytes_eq_should_fail_when_absent, rocket::http::Status::Ok, "expected a response body but no response body was present", .expect_body_bytes_eq(b"abcd"));

    rocket_test!(body_present_should_succeed, "abcd", .expect_body_present());
    rocket_test!(body_present_should_fail_when_absent, rocket::http::Status::Ok, "expected a response body but no response body was present", .expect_body_present());

    rocket_test!(body_absent_should_succeed, rocket::http::Status::Ok, .expect_body_absent());
    rocket_test!(body_absent_should_fail_when_present, "abcd", "expected no response body but a response body was present", .expect_body_absent());
}