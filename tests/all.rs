use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_json::json;

use rocket_test::Resp;

#[path = "rocket_test.rs"]
mod rocket_test;

#[path = "http_client_test.rs"]
mod http_client_test;

#[path = "actix_test.rs"]
mod actix_test;

pub enum Stub {
    StatusAccepted,
    StatusBadRequest,
    StatusConflict,
    StatusCreated,
    StatusForbidden,
    StatusGone,
    StatusMovedPermanently,
    StatusNoContent,
    StatusNotFound,
    StatusOk,
    StatusPartialContent,
    StatusInternalServerError,
    StatusUnauthorized,
    HeaderJson,
    HeaderMany,
    HeaderMulti,
    HeaderOne,
    HeaderText,
    HeaderXml,
    BodyJson,
    BodyJsonAbsent,
    BodyBytes,
    BodyBytesAbsent,
    BodyText,
    BodyTextAbsent,
}

impl Stub {
    fn responses(&self) -> Responses {
        match self {
            Stub::StatusAccepted => Responses(HttpResponse::Accepted().finish(), rocket::http::Status::Accepted.into()),
            Stub::StatusBadRequest => Responses(HttpResponse::BadRequest().finish(), rocket::http::Status::BadRequest.into()),
            Stub::StatusConflict => Responses(HttpResponse::Conflict().finish(), rocket::http::Status::Conflict.into()),
            Stub::StatusCreated => Responses(HttpResponse::Created().finish(), rocket::http::Status::Created.into()),
            Stub::StatusForbidden => Responses(HttpResponse::Forbidden().finish(), rocket::http::Status::Forbidden.into()),
            Stub::StatusGone => Responses(HttpResponse::Gone().finish(), rocket::http::Status::Gone.into()),
            Stub::StatusMovedPermanently => Responses(HttpResponse::MovedPermanently().finish(), rocket::http::Status::MovedPermanently.into()),
            Stub::StatusNoContent => Responses(HttpResponse::NoContent().finish(), rocket::http::Status::NoContent.into()),
            Stub::StatusNotFound => Responses(HttpResponse::NotFound().finish(), rocket::http::Status::NotFound.into()),
            Stub::StatusOk => Responses(HttpResponse::Ok().finish(), rocket::http::Status::Ok.into()),
            Stub::StatusPartialContent => Responses(HttpResponse::PartialContent().finish(), rocket::http::Status::PartialContent.into()),
            Stub::StatusInternalServerError => Responses(HttpResponse::InternalServerError().finish(), rocket::http::Status::InternalServerError.into()),
            Stub::StatusUnauthorized => Responses(HttpResponse::Unauthorized().finish(), rocket::http::Status::Unauthorized.into()),
            Stub::HeaderJson => Responses(HttpResponse::Ok().append_header(("content-type", "application/json")).finish(), vec![("content-type", "application/json")].into()),
            Stub::HeaderMany => Responses(HttpResponse::Ok().append_header(("x-a", "a")).append_header(("x-b", "b")).finish(), vec![("x-a", "a"), ("x-b", "b")].into()),
            Stub::HeaderMulti => Responses(HttpResponse::Ok().append_header(("x-m", "a, b")).finish(), vec![("x-m", vec!["a", "b"])].into()),
            Stub::HeaderOne => Responses(HttpResponse::Ok().append_header(("x-a", "a")).finish(), vec![("x-a", "a")].into()),
            Stub::HeaderText => Responses(HttpResponse::Ok().append_header(("content-type", "text/plain")).finish(), vec![("content-type", "text/plain")].into()),
            Stub::HeaderXml => Responses(HttpResponse::Ok().append_header(("content-type", "application/xml")).finish(), vec![("content-type", "application/xml")].into()),
            Stub::BodyJson => Responses(HttpResponse::Ok().body(json!({"a": "b"}).to_string()), json!({"a": "b"}).into()),
            Stub::BodyJsonAbsent => Responses(HttpResponse::Ok().finish(), rocket::http::Status::Ok.into()),
            Stub::BodyBytes => Responses(HttpResponse::Ok().body("abcd"), "abcd".into()),
            Stub::BodyBytesAbsent => Responses(HttpResponse::Ok().finish(), rocket::http::Status::Ok.into()),
            Stub::BodyText => Responses(HttpResponse::Ok().body("abcd"), "abcd".into()),
            Stub::BodyTextAbsent => Responses(HttpResponse::Ok().finish(), rocket::http::Status::Ok.into()),
        }
    }
}

pub struct Responses(pub HttpResponse, pub Resp);

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct TestBody {
    pub a: String,
}

#[macro_export]
macro_rules! asserhttp_test {
    ($fn_name:ident, $stub:literal, $resp:expr, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        $(crate::http_client_test!($fn_name, $stub, $( .$meth($($arg),*) )* );)+
        $(crate::actix_test!($fn_name, $resp.0, $( .$meth($($arg),*) )* );)+
        $(crate::rocket_test!($fn_name, $resp.1, $( .$meth($($arg),*) )* );)+
    };
    ($fn_name:ident, $stub:literal, $resp:expr, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        $(crate::http_client_test!($fn_name, $stub, $panic_msg, $( .$meth($($arg),*) )* );)+
        $(crate::actix_test!($fn_name, $resp.0, $panic_msg, $( .$meth($($arg),*) )* );)+
        $(crate::rocket_test!($fn_name, $resp.1, $panic_msg, $( .$meth($($arg),*) )* );)+
    };
}

mod smoke {
    use super::Stub::*;

    asserhttp_test!(simple_should_succeed, "status/ok.json", StatusOk.responses(), .expect_status_eq(200));
    asserhttp_test!(simple_should_fail, "status/ok.json", StatusOk.responses(), "", .expect_status_eq(100));
    asserhttp_test!(simple_should_chain, "status/ok.json", StatusOk.responses(), .expect_status_eq(200).expect_status_eq(200));
}

mod status {
    use super::Stub::*;

    asserhttp_test!(status_eq_should_succeed, "status/ok.json", StatusOk.responses(), .expect_status_eq(200));
    asserhttp_test!(status_eq_enum_should_succeed, "status/ok.json", StatusOk.responses(), .expect_status_eq(Status::Ok));
    asserhttp_test!(status_eq_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be '100' but was '200'", .expect_status_eq(100));

    asserhttp_test!(status_ok_should_succeed, "status/ok.json", StatusOk.responses(), .expect_status_ok());
    asserhttp_test!(status_ok_should_fail, "status/created.json", StatusCreated.responses(), "expected status to be '200' but was '201'", .expect_status_ok());
    asserhttp_test!(status_created_should_succeed, "status/created.json", StatusCreated.responses(), .expect_status_created());
    asserhttp_test!(status_created_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be '201' but was '200'", .expect_status_created());
    asserhttp_test!(status_accepted_should_succeed, "status/accepted.json", StatusAccepted.responses(), .expect_status_accepted());
    asserhttp_test!(status_accepted_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be '202' but was '200'", .expect_status_accepted());
    asserhttp_test!(status_no_content_should_succeed, "status/no-content.json", StatusNoContent.responses(), .expect_status_no_content());
    asserhttp_test!(status_no_content_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be '204' but was '200'", .expect_status_no_content());
    asserhttp_test!(status_partial_content_should_succeed, "status/partial-content.json", StatusPartialContent.responses(), .expect_status_partial_content());
    asserhttp_test!(status_partial_content_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be '206' but was '200'", .expect_status_partial_content());
    asserhttp_test!(status_bad_request_should_succeed, "status/bad-request.json", StatusBadRequest.responses(), .expect_status_bad_request());
    asserhttp_test!(status_bad_request_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be '400' but was '200'", .expect_status_bad_request());
    asserhttp_test!(status_unauthorized_should_succeed, "status/unauthorized.json", StatusUnauthorized.responses(), .expect_status_unauthorized());
    asserhttp_test!(status_unauthorized_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be '401' but was '200'", .expect_status_unauthorized());
    asserhttp_test!(status_forbidden_should_succeed, "status/forbidden.json", StatusForbidden.responses(), .expect_status_forbidden());
    asserhttp_test!(status_forbidden_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be '403' but was '200'", .expect_status_forbidden());
    asserhttp_test!(status_not_found_should_succeed, "status/not-found.json", StatusNotFound.responses(), .expect_status_not_found());
    asserhttp_test!(status_not_found_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be '404' but was '200'", .expect_status_not_found());
    asserhttp_test!(status_conflict_should_succeed, "status/conflict.json", StatusConflict.responses(), .expect_status_conflict());
    asserhttp_test!(status_conflict_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be '409' but was '200'", .expect_status_conflict());
    asserhttp_test!(status_gone_should_succeed, "status/gone.json", StatusGone.responses(), .expect_status_gone());
    asserhttp_test!(status_gone_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be '410' but was '200'", .expect_status_gone());
    asserhttp_test!(status_internal_server_error_should_succeed, "status/server-error.json", StatusInternalServerError.responses(), .expect_status_internal_server_error());
    asserhttp_test!(status_internal_server_error_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be '500' but was '200'", .expect_status_internal_server_error());

    asserhttp_test!(status_range_should_succeed, "status/created.json", StatusCreated.responses(), .expect_status_in_range(200, 300));
    asserhttp_test!(status_range_should_fail_when_higher, "status/server-error.json", StatusInternalServerError.responses(), "expected status to be in [300;400[ but was '500'", .expect_status_in_range(300, 400));
    asserhttp_test!(status_range_should_fail_when_lower, "status/ok.json", StatusOk.responses(), "expected status to be in [300;400[ but was '200'", .expect_status_in_range(300, 400));
    asserhttp_test!(status_range_should_succeed_in_inclusive_lower_range, "status/ok.json", StatusOk.responses(), .expect_status_in_range(200, 202));
    asserhttp_test!(status_range_should_fail_when_in_exclusive_upper_range, "status/accepted.json", StatusAccepted.responses(), "expected status to be in [200;202[ but was '202'", .expect_status_in_range(200, 202));

    asserhttp_test!(status_success_should_succeed, "status/ok.json", StatusOk.responses(), .expect_status_success());
    asserhttp_test!(status_success_should_fail, "status/bad-request.json", StatusBadRequest.responses(), "expected status to be in [200;300[ but was '400'", .expect_status_success());
    asserhttp_test!(status_redirection_should_succeed, "status/moved-permanently.json", StatusMovedPermanently.responses(), .expect_status_redirection());
    asserhttp_test!(status_redirection_should_fail, "status/server-error.json", StatusInternalServerError.responses(), "expected status to be in [300;400[ but was '500'", .expect_status_redirection());
    asserhttp_test!(status_client_error_should_succeed, "status/bad-request.json", StatusBadRequest.responses(), .expect_status_client_error());
    asserhttp_test!(status_client_error_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be in [400;500[ but was '200'", .expect_status_client_error());
    asserhttp_test!(status_server_error_should_succeed, "status/server-error.json", StatusInternalServerError.responses(), .expect_status_server_error());
    asserhttp_test!(status_server_error_should_fail, "status/ok.json", StatusOk.responses(), "expected status to be in [500;600[ but was '200'", .expect_status_server_error());
}

mod header {
    use super::Stub::*;

    asserhttp_test!(header_eq_should_succeed, "header/one.json", HeaderOne.responses(), .expect_header("x-a", "a"));
    asserhttp_test!(header_eq_should_match_key_ignoring_case, "header/one.json", HeaderOne.responses(), .expect_header("X-A", "a"));
    asserhttp_test!(header_eq_should_fail_because_value_case_sensitive, "header/one.json", HeaderOne.responses(), "expected header 'x-a' to be equal to 'A' but was 'a'", .expect_header("x-a", "A"));
    asserhttp_test!(header_eq_should_fail_when_wrong_key, "header/one.json", HeaderOne.responses(), "expected one header named 'x-b' but none found", .expect_header("x-b", "a"));
    asserhttp_test!(header_eq_should_fail_when_wrong_value, "header/one.json", HeaderOne.responses(), "expected header 'x-a' to be equal to 'b' but was 'a'", .expect_header("x-a", "b"));
    asserhttp_test!(header_eq_many_should_succeed, "header/many.json", HeaderMany.responses(), .expect_header("x-a", "a").expect_header("x-b", "b"));
    asserhttp_test!(header_eq_should_fail_when_multivalued, "header/multi.json", HeaderMulti.responses(), "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.", .expect_header("x-m", "a"));

    asserhttp_test!(header_closure_should_succeed, "header/one.json", HeaderOne.responses(), .expect_header("x-a", |h| assert_eq!(h, "a")));
    asserhttp_test!(header_closure_should_fail, "header/one.json", HeaderOne.responses(), "", .expect_header("x-a", |h| assert_eq!(h, "b")));

    asserhttp_test!(header_multi_should_succeed, "header/multi.json", HeaderMulti.responses(), .expect_headers("x-m", ["a", "b"]));
    asserhttp_test!(header_multi_should_fail_when_key_missing, "header/multi.json", HeaderMulti.responses(), "expected one header named 'x-b' but none found", .expect_headers("x-b", ["a", "b"]));
    asserhttp_test!(header_multi_should_fail_when_one_value_missing, "header/multi.json", HeaderMulti.responses(), "expected header 'x-m' to contain values '[\"a\"]' but contained '[\"a\", \"b\"]'", .expect_headers("x-m", ["a"]));
    asserhttp_test!(header_multi_should_fail_when_one_value_not_eq, "header/multi.json", HeaderMulti.responses(), "expected header 'x-m' to contain values '[\"a\", \"c\"]' but contained '[\"a\", \"b\"]'", .expect_headers("x-m", ["a", "c"]));
    asserhttp_test!(header_multi_should_fail_when_one_expected_value_missing, "header/multi.json", HeaderMulti.responses(), "expected header 'x-m' to contain values '[\"a\", \"b\", \"c\"]' but contained '[\"a\", \"b\"]'", .expect_headers("x-m", ["a", "b", "c"]));
    asserhttp_test!(header_multi_should_fail_when_no_value_expected, "header/multi.json", HeaderMulti.responses(), "no value expected for header 'x-m'. Use 'expect_header_present' instead", .expect_headers("x-m", []));

    asserhttp_test!(header_present_should_succeed, "header/one.json", HeaderOne.responses(), .expect_header_present("x-a"));
    asserhttp_test!(header_present_should_succeed_ignoring_case, "header/one.json", HeaderOne.responses(), .expect_header_present("X-A"));
    asserhttp_test!(header_present_should_fail_when_absent, "header/one.json", HeaderOne.responses(), "expected one header named 'x-b' but none found", .expect_header_present("x-b"));

    asserhttp_test!(header_absent_should_succeed, "header/one.json", HeaderOne.responses(), .expect_header_absent("x-b"));
    asserhttp_test!(header_absent_should_fail_when_present, "header/one.json", HeaderOne.responses(), "expected no header named 'x-a' but one found", .expect_header_absent("x-a"));
    asserhttp_test!(header_absent_should_fail_when_present_ignoring_case, "header/one.json", HeaderOne.responses(), "expected no header named 'X-A' but one found", .expect_header_absent("X-A"));

    asserhttp_test!(header_content_type_json_should_succeed, "header/json.json", HeaderJson.responses(), .expect_content_type_json());
    asserhttp_test!(header_content_type_json_should_fail, "header/xml.json", HeaderXml.responses(), "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'", .expect_content_type_json());

    asserhttp_test!(header_content_type_text_should_succeed, "header/text.json", HeaderText.responses(), .expect_content_type_text());
    asserhttp_test!(header_content_type_text_should_fail, "header/xml.json", HeaderXml.responses(), "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'", .expect_content_type_text());
}

mod body {
    use serde_json::{json, Value};

    use super::{*, Stub::*};

    asserhttp_test!(body_json_should_succeed, "body/json/value.json", BodyJson.responses(), .expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"}))));
    asserhttp_test!(body_json_struct_should_succeed, "body/json/value.json", BodyJson.responses(), .expect_body_json(|b: TestBody| assert_eq!(b, TestBody { a: String::from("b") })));
    asserhttp_test!(body_json_should_fail_when_closure_fails, "body/json/value.json", BodyJson.responses(), "", .expect_body_json(|b: Value| assert_eq!(b, json!({"a": "c"}))));

    asserhttp_test!(body_json_eq_should_succeed, "body/json/value.json", BodyJson.responses(), .expect_body_json_eq(json!({"a": "b"})));
    asserhttp_test!(body_json_struct_eq_should_succeed, "body/json/value.json", BodyJson.responses(), .expect_body_json_eq(TestBody { a: String::from("b") }));
    asserhttp_test!(body_json_eq_should_fail_when_not_eq, "body/json/value.json", BodyJson.responses(), "expected json body '{\"a\":\"b\"}' to be equal to '{\"a\":\"c\"}' but was not", .expect_body_json_eq(json!({"a": "c"})));
    asserhttp_test!(body_json_eq_should_fail_when_absent, "body/json/absent.json", BodyJsonAbsent.responses(), "expected a json body but no response body was present", .expect_body_json_eq(json!({"a": "b"})));

    asserhttp_test!(body_text_should_succeed, "body/text/value.json", BodyText.responses(), .expect_body_text(|b| assert_eq!(b, String::from("abcd"))));
    asserhttp_test!(body_text_should_fail_when_closure_fails, "body/text/value.json", BodyText.responses(), "", .expect_body_text(|b| assert_eq!(b, String::from("dcba"))));

    asserhttp_test!(body_text_eq_should_succeed, "body/text/value.json", BodyText.responses(), .expect_body_text_eq("abcd"));
    asserhttp_test!(body_text_eq_should_fail_when_not_eq, "body/text/value.json", BodyText.responses(), "expected text body 'abcd' to be equal to 'dcba' but was not", .expect_body_text_eq("dcba"));
    asserhttp_test!(body_text_eq_should_fail_when_absent, "body/text/absent.json", BodyTextAbsent.responses(), "expected a text body but no response body was present", .expect_body_text_eq("abcd"));

    asserhttp_test!(body_text_regex_should_succeed, "body/text/value.json", BodyText.responses(), .expect_body_text_matches("[a-d]+"));
    asserhttp_test!(body_text_regex_should_fail_when_does_not_match, "body/text/value.json", BodyText.responses(), "expected text body 'abcd' to match regex '[e-h]+' but did not", .expect_body_text_matches("[e-h]+"));
    asserhttp_test!(body_text_regex_should_fail_when_absent, "body/text/absent.json", BodyTextAbsent.responses(), "expected a text body but no response body was present", .expect_body_text_matches("[a-d]+"));

    asserhttp_test!(body_bytes_should_succeed, "body/bytes/value.json", BodyBytes.responses(), .expect_body_bytes(|b| assert_eq!(b, b"abcd")));
    asserhttp_test!(body_bytes_should_fail_when_closure_fails, "body/bytes/value.json", BodyBytes.responses(), "", .expect_body_bytes(|b| assert_eq!(b, b"dcba")));

    asserhttp_test!(body_bytes_eq_should_succeed, "body/bytes/value.json", BodyBytes.responses(), .expect_body_bytes_eq(b"abcd"));
    asserhttp_test!(body_bytes_eq_should_fail_when_not_eq, "body/bytes/value.json", BodyBytes.responses(), "expected body '[97, 98, 99, 100]' to be equal to '[100, 99, 98, 97]' but was not", .expect_body_bytes_eq(b"dcba"));
    asserhttp_test!(body_bytes_eq_should_fail_when_absent, "body/bytes/absent.json", BodyBytesAbsent.responses(), "expected a response body but no response body was present", .expect_body_bytes_eq(b"abcd"));

    asserhttp_test!(body_present_should_succeed, "body/bytes/value.json", BodyBytes.responses(), .expect_body_present());
    asserhttp_test!(body_present_should_fail_when_absent, "body/bytes/absent.json", BodyBytesAbsent.responses(), "expected a response body but no response body was present", .expect_body_present());

    asserhttp_test!(body_absent_should_succeed, "body/bytes/absent.json", BodyBytesAbsent.responses(), .expect_body_absent());
    asserhttp_test!(body_absent_should_fail_when_present, "body/bytes/value.json", BodyBytes.responses(), "expected no response body but a response body was present", .expect_body_absent());
}