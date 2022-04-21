use utils::TestBody;

#[path = "./utils.rs"]
mod utils;

// For actix
#[macro_export]
macro_rules! actix_test {
    ($fn_name:ident, $init:expr, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // unit
            #[actix_web::test]
            async fn [<actix_unit_ $fn_name>]() {
                use asserhttp::*;
                use actix_web::{HttpRequest, HttpResponse, test::TestRequest};
                async fn ok(_: HttpRequest) -> HttpResponse { $init }
                $(ok(TestRequest::get().to_http_request()).await$( .$meth($($arg),*) )+;)+
            }

            // integration
            #[actix_web::test]
            async fn [<actix_integration_ $fn_name>]() {
                use asserhttp::*;
                use actix_web::{App, HttpResponse, test::{call_service, init_service, TestRequest}, web};
                let app = App::new().route("/", web::get().to(|| async { $init }));
                $(call_service(&init_service(app).await, TestRequest::get().to_request()).await$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $init:expr, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // unit
            #[should_panic(expected = $panic_msg)]
            #[actix_web::test]
            async fn [<actix_unit_ $fn_name>]() {
                use asserhttp::*;
                use actix_web::{HttpRequest, HttpResponse, test::TestRequest};
                async fn ok(_: HttpRequest) -> HttpResponse { $init }
                $(ok(TestRequest::get().to_http_request()).await$( .$meth($($arg),*) )+;)+
            }

            // integration
            #[should_panic(expected = $panic_msg)]
            #[actix_web::test]
            async fn [<actix_integration_ $fn_name>]() {
                use asserhttp::*;
                use actix_web::{App, HttpResponse, test::{call_service, init_service, TestRequest}, web};
                let app = App::new().route("/", web::get().to(|| async { $init }));
                $(call_service(&init_service(app).await, TestRequest::get().to_request()).await$( .$meth($($arg),*) )+;)+
            }
        }
    };
}

mod smoke {
    actix_test!(simple_should_succeed, HttpResponse::Ok().finish(), .expect_status_eq(200));
    actix_test!(simple_should_fail, HttpResponse::Ok().finish(), "", .expect_status_eq(100));
    actix_test!(simple_should_chain, HttpResponse::Ok().finish(), .expect_status_eq(200).expect_status_eq(200));
}

mod status {
    actix_test!(status_eq_should_succeed, HttpResponse::Ok().finish(), .expect_status_eq(200));
    actix_test!(status_eq_enum_should_succeed, HttpResponse::Ok().finish(), .expect_status_eq(Status::Ok));
    actix_test!(status_eq_should_fail, HttpResponse::Ok().finish(), "expected status to be '100' but was '200'", .expect_status_eq(100));

    actix_test!(status_ok_should_succeed, HttpResponse::Ok().finish(), .expect_status_ok());
    actix_test!(status_ok_should_fail, HttpResponse::Created().finish(), "expected status to be '200' but was '201'", .expect_status_ok());
    actix_test!(status_created_should_succeed, HttpResponse::Created().finish(), .expect_status_created());
    actix_test!(status_created_should_fail, HttpResponse::Ok().finish(), "expected status to be '201' but was '200'", .expect_status_created());
    actix_test!(status_accepted_should_succeed, HttpResponse::Accepted().finish(), .expect_status_accepted());
    actix_test!(status_accepted_should_fail, HttpResponse::Ok().finish(), "expected status to be '202' but was '200'", .expect_status_accepted());
    actix_test!(status_no_content_should_succeed, HttpResponse::NoContent().finish(), .expect_status_no_content());
    actix_test!(status_no_content_should_fail, HttpResponse::Ok().finish(), "expected status to be '204' but was '200'", .expect_status_no_content());
    actix_test!(status_partial_content_should_succeed, HttpResponse::PartialContent().finish(), .expect_status_partial_content());
    actix_test!(status_partial_content_should_fail, HttpResponse::Ok().finish(), "expected status to be '206' but was '200'", .expect_status_partial_content());
    actix_test!(status_bad_request_should_succeed, HttpResponse::BadRequest().finish(), .expect_status_bad_request());
    actix_test!(status_bad_request_should_fail, HttpResponse::Ok().finish(), "expected status to be '400' but was '200'", .expect_status_bad_request());
    actix_test!(status_unauthorized_should_succeed, HttpResponse::Unauthorized().finish(), .expect_status_unauthorized());
    actix_test!(status_unauthorized_should_fail, HttpResponse::Ok().finish(), "expected status to be '401' but was '200'", .expect_status_unauthorized());
    actix_test!(status_forbidden_should_succeed, HttpResponse::Forbidden().finish(), .expect_status_forbidden());
    actix_test!(status_forbidden_should_fail, HttpResponse::Ok().finish(), "expected status to be '403' but was '200'", .expect_status_forbidden());
    actix_test!(status_not_found_should_succeed, HttpResponse::NotFound().finish(), .expect_status_not_found());
    actix_test!(status_not_found_should_fail, HttpResponse::Ok().finish(), "expected status to be '404' but was '200'", .expect_status_not_found());
    actix_test!(status_conflict_should_succeed, HttpResponse::Conflict().finish(), .expect_status_conflict());
    actix_test!(status_conflict_should_fail, HttpResponse::Ok().finish(), "expected status to be '409' but was '200'", .expect_status_conflict());
    actix_test!(status_gone_should_succeed, HttpResponse::Gone().finish(), .expect_status_gone());
    actix_test!(status_gone_should_fail, HttpResponse::Ok().finish(), "expected status to be '410' but was '200'", .expect_status_gone());
    actix_test!(status_internal_server_error_should_succeed, HttpResponse::InternalServerError().finish(), .expect_status_internal_server_error());
    actix_test!(status_internal_server_error_should_fail, HttpResponse::Ok().finish(), "expected status to be '500' but was '200'", .expect_status_internal_server_error());

    actix_test!(status_range_should_succeed, HttpResponse::Created().finish(), .expect_status_in_range(200, 300));
    actix_test!(status_range_should_fail_when_higher, HttpResponse::InternalServerError().finish(), "expected status to be in [300;400[ but was '500'", .expect_status_in_range(300, 400));
    actix_test!(status_range_should_fail_when_lower, HttpResponse::Ok().finish(), "expected status to be in [300;400[ but was '200'", .expect_status_in_range(300, 400));
    actix_test!(status_range_should_succeed_in_inclusive_lower_range, HttpResponse::Ok().finish(), .expect_status_in_range(200, 202));
    actix_test!(status_range_should_fail_when_in_exclusive_upper_range, HttpResponse::Accepted().finish(), "expected status to be in [200;202[ but was '202'", .expect_status_in_range(200, 202));

    actix_test!(status_success_should_succeed, HttpResponse::Ok().finish(), .expect_status_success());
    actix_test!(status_success_should_fail, HttpResponse::BadRequest().finish(), "expected status to be in [200;300[ but was '400'", .expect_status_success());
    actix_test!(status_redirection_should_succeed, HttpResponse::MovedPermanently().finish(), .expect_status_redirection());
    actix_test!(status_redirection_should_fail, HttpResponse::InternalServerError().finish(), "expected status to be in [300;400[ but was '500'", .expect_status_redirection());
    actix_test!(status_client_error_should_succeed, HttpResponse::BadRequest().finish(), .expect_status_client_error());
    actix_test!(status_client_error_should_fail, HttpResponse::Ok().finish(), "expected status to be in [400;500[ but was '200'", .expect_status_client_error());
    actix_test!(status_server_error_should_succeed, HttpResponse::InternalServerError().finish(), .expect_status_server_error());
    actix_test!(status_server_error_should_fail, HttpResponse::Ok().finish(), "expected status to be in [500;600[ but was '200'", .expect_status_server_error());
}

mod header {
    actix_test!(header_eq_should_succeed, HttpResponse::Ok().append_header(("x-a", "a")).finish(), .expect_header("x-a", "a"));
    actix_test!(header_eq_should_match_key_ignoring_case, HttpResponse::Ok().append_header(("x-a", "a")).finish(), .expect_header("X-A", "a"));
    actix_test!(header_eq_should_fail_because_value_case_sensitive, HttpResponse::Ok().append_header(("x-a", "a")).finish(), "expected header 'x-a' to be equal to 'A' but was 'a'", .expect_header("x-a", "A"));
    actix_test!(header_eq_should_fail_when_wrong_key, HttpResponse::Ok().append_header(("x-a", "a")).finish(), "expected one header named 'x-b' but none found", .expect_header("x-b", "a"));
    actix_test!(header_eq_should_fail_when_wrong_value, HttpResponse::Ok().append_header(("x-a", "a")).finish(), "expected header 'x-a' to be equal to 'b' but was 'a'", .expect_header("x-a", "b"));
    actix_test!(header_eq_many_should_succeed, HttpResponse::Ok().append_header(("x-a", "a")).append_header(("x-b", "b")).finish(), .expect_header("x-a", "a").expect_header("x-b", "b"));
    actix_test!(header_eq_should_fail_when_multivalued, HttpResponse::Ok().append_header(("x-m", "a, b")).finish(), "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.", .expect_header("x-m", "a"));

    actix_test!(header_multi_should_succeed, HttpResponse::Ok().append_header(("x-m", "a, b")).finish(), .expect_headers("x-m", ["a", "b"]));
    actix_test!(header_multi_should_fail_when_key_missing, HttpResponse::Ok().append_header(("x-m", "a, b")).finish(), "expected one header named 'x-b' but none found", .expect_headers("x-b", ["a", "b"]));
    actix_test!(header_multi_should_fail_when_one_value_missing, HttpResponse::Ok().append_header(("x-m", "a, b")).finish(), "expected header 'x-m' to contain values '[\"a\"]' but contained '[\"a\", \"b\"]'", .expect_headers("x-m", ["a"]));
    actix_test!(header_multi_should_fail_when_one_value_not_eq, HttpResponse::Ok().append_header(("x-m", "a, b")).finish(), "expected header 'x-m' to contain values '[\"a\", \"c\"]' but contained '[\"a\", \"b\"]'", .expect_headers("x-m", ["a", "c"]));
    actix_test!(header_multi_should_fail_when_one_expected_value_missing, HttpResponse::Ok().append_header(("x-m", "a, b")).finish(), "expected header 'x-m' to contain values '[\"a\", \"b\", \"c\"]' but contained '[\"a\", \"b\"]'", .expect_headers("x-m", ["a", "b", "c"]));
    actix_test!(header_multi_should_fail_when_no_value_expected, HttpResponse::Ok().append_header(("x-m", "a, b")).finish(), "no value expected for header 'x-m'. Use 'expect_header_present' instead", .expect_headers("x-m", []));

    actix_test!(header_present_should_succeed, HttpResponse::Ok().append_header(("x-a", "a")).finish(), .expect_header_present("x-a"));
    actix_test!(header_present_should_succeed_ignoring_case, HttpResponse::Ok().append_header(("x-a", "a")).finish(), .expect_header_present("X-A"));
    actix_test!(header_present_should_fail_when_absent, HttpResponse::Ok().append_header(("x-a", "a")).finish(), "expected one header named 'x-b' but none found", .expect_header_present("x-b"));

    actix_test!(header_absent_should_succeed, HttpResponse::Ok().append_header(("x-a", "a")).finish(), .expect_header_absent("x-b"));
    actix_test!(header_absent_should_fail_when_present, HttpResponse::Ok().append_header(("x-a", "a")).finish(), "expected no header named 'x-a' but one found", .expect_header_absent("x-a"));
    actix_test!(header_absent_should_fail_when_present_ignoring_case, HttpResponse::Ok().append_header(("x-a", "a")).finish(), "expected no header named 'x-a' but one found", .expect_header_absent("X-A"));

    actix_test!(header_content_type_json_should_succeed, HttpResponse::Ok().append_header(("content-type", "application/json")).finish(), .expect_content_type_json());
    actix_test!(header_content_type_json_should_fail, HttpResponse::Ok().append_header(("content-type", "application/xml")).finish(), "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'", .expect_content_type_json());

    actix_test!(header_content_type_text_should_succeed, HttpResponse::Ok().append_header(("content-type", "text/plain")).finish(), .expect_content_type_text());
    actix_test!(header_content_type_text_should_fail, HttpResponse::Ok().append_header(("content-type", "application/xml")).finish(), "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'", .expect_content_type_text());
}

mod body {
    use serde_json::{json, Value};

    use super::*;

    actix_test!(body_json_should_succeed, HttpResponse::Ok().body(json!({"a": "b"}).to_string()), .expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"}))));
    actix_test!(body_json_struct_should_succeed, HttpResponse::Ok().body(json!({"a": "b"}).to_string()), .expect_body_json(|b: TestBody| assert_eq!(b, TestBody { a: String::from("b") })));
    actix_test!(body_json_should_fail_when_closure_fails, HttpResponse::Ok().body(json!({"a": "b"}).to_string()), "", .expect_body_json(|b: Value| assert_eq!(b, json!({"a": "c"}))));

    actix_test!(body_json_eq_should_succeed, HttpResponse::Ok().body(json!({"a": "b"}).to_string()), .expect_body_json_eq(json!({"a": "b"})));
    actix_test!(body_json_struct_eq_should_succeed, HttpResponse::Ok().body(json!({"a": "b"}).to_string()), .expect_body_json_eq(TestBody { a: String::from("b") }));
    actix_test!(body_json_eq_should_fail_when_not_eq, HttpResponse::Ok().body(json!({"a": "b"}).to_string()), "expected json body '{\"a\":\"b\"}' to be equal to '{\"a\":\"c\"}' but was not", .expect_body_json_eq(json!({"a": "c"})));
    actix_test!(body_json_eq_should_fail_when_absent, HttpResponse::Ok().finish(), "expected a json body but no response body was present", .expect_body_json_eq(json!({"a": "b"})));

    actix_test!(body_text_should_succeed, HttpResponse::Ok().body("abcd"), .expect_body_text(|b| assert_eq!(b, String::from("abcd"))));
    actix_test!(body_text_should_fail_when_closure_fails, HttpResponse::Ok().body("abcd"), "", .expect_body_text(|b| assert_eq!(b, String::from("dcba"))));

    actix_test!(body_text_eq_should_succeed, HttpResponse::Ok().body("abcd"), .expect_body_text_eq("abcd"));
    actix_test!(body_text_eq_should_fail_when_not_eq, HttpResponse::Ok().body("abcd"), "expected text body 'abcd' to be equal to 'dcba' but was not", .expect_body_text_eq("dcba"));
    actix_test!(body_text_eq_should_fail_when_absent, HttpResponse::Ok().finish(), "expected a text body but no response body was present", .expect_body_text_eq("abcd"));

    actix_test!(body_text_regex_should_succeed, HttpResponse::Ok().body("abcd"), .expect_body_text_matches("[a-d]+"));
    actix_test!(body_text_regex_should_fail_when_does_not_match, HttpResponse::Ok().body("abcd"), "expected text body 'abcd' to match regex '[e-h]+' but did not", .expect_body_text_matches("[e-h]+"));
    actix_test!(body_text_regex_should_fail_when_absent, HttpResponse::Ok().finish(), "expected a text body but no response body was present", .expect_body_text_matches("[a-d]+"));

    actix_test!(body_bytes_should_succeed, HttpResponse::Ok().body("abcd"), .expect_body_bytes(|b| assert_eq!(b, b"abcd")));
    actix_test!(body_bytes_should_fail_when_closure_fails, HttpResponse::Ok().body("abcd"), "", .expect_body_bytes(|b| assert_eq!(b, b"dcba")));

    actix_test!(body_bytes_eq_should_succeed, HttpResponse::Ok().body("abcd"), .expect_body_bytes_eq(b"abcd"));
    actix_test!(body_bytes_eq_should_fail_when_not_eq, HttpResponse::Ok().body("abcd"), "expected body '[97, 98, 99, 100]' to be equal to '[100, 99, 98, 97]' but was not", .expect_body_bytes_eq(b"dcba"));
    actix_test!(body_bytes_eq_should_fail_when_absent, HttpResponse::Ok().finish(), "expected a response body but no response body was present", .expect_body_bytes_eq(b"abcd"));

    actix_test!(body_present_should_succeed, HttpResponse::Ok().body("abcd"), .expect_body_present());
    actix_test!(body_present_should_fail_when_absent, HttpResponse::Ok().finish(), "expected a response body but no response body was present", .expect_body_present());

    actix_test!(body_absent_should_succeed, HttpResponse::Ok().finish(), .expect_body_absent());
    actix_test!(body_absent_should_fail_when_present, HttpResponse::Ok().body("abcd"), "expected no response body but a response body was present", .expect_body_absent());
}