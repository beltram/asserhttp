use utils::TestBody;

#[path = "./utils.rs"]
mod utils;

// For reqwest, surf, isahc, hyper & awc
#[macro_export]
macro_rules! http_client_test {
    ($fn_name:ident, $stub:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // reqwest blocking
            #[test]
            #[stubr::mock($stub)]
            fn [<reqwest_blocking_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::blocking::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[test]
            #[stubr::mock($stub)]
            fn [<reqwest_blocking_result_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::blocking::get(stubr.uri())$( .$meth($($arg),*) )+;)+
            }

            // reqwest async
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<reqwest_async_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::get(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<reqwest_async_result_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::get(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }

            // surf
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<surf_ $fn_name>]() {
                use asserhttp::*;
                $(surf::get(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<surf_result_ $fn_name>]() {
                use asserhttp::*;
                $(surf::get(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }

            // hyper
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<hyper_ $fn_name>]() {
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await$( .$meth($($arg),*) )+;)+
            }
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<hyper_result_ $fn_name>]() {
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }

            // awc
            #[actix_web::test]
            #[stubr::mock($stub)]
            async fn [<awc_ $fn_name>]() {
                use asserhttp::*;
                $(awc::Client::default().get(stubr.uri()).send().await$( .$meth($($arg),*) )+;)+
            }
            #[actix_web::test]
            #[stubr::mock($stub)]
            async fn [<awc_result_ $fn_name>]() {
                use asserhttp::*;
                $(awc::Client::default().get(stubr.uri()).send().await.unwrap()$( .$meth($($arg),*) )+;)+
            }

            // isahc blocking
            #[test]
            #[stubr::mock($stub)]
            fn [<isahc_blocking_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[test]
            #[stubr::mock($stub)]
            fn [<isahc_blocking_result_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get(stubr.uri())$( .$meth($($arg),*) )+;)+
            }

            // isahc async
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<isahc_async_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get_async(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }
            #[tokio::test]
            #[stubr::mock($stub)]
            async fn [<isahc_async_result_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get_async(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
        }
    };
    ($fn_name:ident, $stub:literal, $panic_msg:literal, $($(.$meth:ident($( $arg:expr ),*))+),+) => {
        paste::paste! {
            // reqwest blocking
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<reqwest_blocking_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::blocking::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<reqwest_blocking_result_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::blocking::get(stubr.uri())$( .$meth($($arg),*) )+;)+
            }

            // reqwest async
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<reqwest_async_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::get(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<reqwest_async_result_ $fn_name>]() {
                use asserhttp::*;
                $(reqwest::get(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }

            // surf
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<surf_ $fn_name>]() {
                use asserhttp::*;
                $(surf::get(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<surf_result_ $fn_name>]() {
                use asserhttp::*;
                $(surf::get(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }

            // hyper
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<hyper_ $fn_name>]() {
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<hyper_result_ $fn_name>]() {
                use asserhttp::*;
                $(hyper::Client::new().get(stubr.uri().parse().unwrap()).await$( .$meth($($arg),*) )+;)+
            }

            // awc
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[actix_web::test]
            async fn [<awc_ $fn_name>]() {
                use asserhttp::*;
                $(awc::Client::default().get(stubr.uri()).send().await$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[actix_web::test]
            async fn [<awc_result_ $fn_name>]() {
                use asserhttp::*;
                $(awc::Client::default().get(stubr.uri()).send().await.unwrap()$( .$meth($($arg),*) )+;)+
            }

            // isahc blocking
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<isahc_blocking_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get(stubr.uri()).unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[test]
            fn [<isahc_blocking_result_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get(stubr.uri())$( .$meth($($arg),*) )+;)+
            }

            // isahc async
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<isahc_async_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get_async(stubr.uri()).await.unwrap()$( .$meth($($arg),*) )+;)+
            }
            #[should_panic(expected = $panic_msg)]
            #[stubr::mock($stub)]
            #[tokio::test]
            async fn [<isahc_async_result_ $fn_name>]() {
                use asserhttp::*;
                $(isahc::get_async(stubr.uri()).await$( .$meth($($arg),*) )+;)+
            }
        }
    };
}

mod smoke {
    http_client_test!(simple_should_succeed, "ping.json", .expect_status_eq(200));
    http_client_test!(simple_should_fail, "ping.json", "", .expect_status_eq(100));
    http_client_test!(simple_should_chain, "ping.json", .expect_status_eq(200).expect_status_eq(200));
}

mod status {
    http_client_test!(status_eq_should_succeed, "status/eq.json", .expect_status_eq(200));
    http_client_test!(status_eq_enum_should_succeed, "status/eq.json", .expect_status_eq(Status::Ok));
    http_client_test!(status_eq_should_fail, "status/eq.json", "expected status to be '100' but was '200'", .expect_status_eq(100));

    http_client_test!(status_ok_should_succeed, "status/ok.json", .expect_status_ok());
    http_client_test!(status_ok_should_fail, "status/created.json", "expected status to be '200' but was '201'", .expect_status_ok());
    http_client_test!(status_created_should_succeed, "status/created.json", .expect_status_created());
    http_client_test!(status_created_should_fail, "status/ok.json", "expected status to be '201' but was '200'", .expect_status_created());
    http_client_test!(status_accepted_should_succeed, "status/accepted.json", .expect_status_accepted());
    http_client_test!(status_accepted_should_fail, "status/ok.json", "expected status to be '202' but was '200'", .expect_status_accepted());
    http_client_test!(status_no_content_should_succeed, "status/no-content.json", .expect_status_no_content());
    http_client_test!(status_no_content_should_fail, "status/ok.json", "expected status to be '204' but was '200'", .expect_status_no_content());
    http_client_test!(status_partial_content_should_succeed, "status/partial-content.json", .expect_status_partial_content());
    http_client_test!(status_partial_content_should_fail, "status/ok.json", "expected status to be '206' but was '200'", .expect_status_partial_content());
    http_client_test!(status_bad_request_should_succeed, "status/bad-request.json", .expect_status_bad_request());
    http_client_test!(status_bad_request_should_fail, "status/ok.json", "expected status to be '400' but was '200'", .expect_status_bad_request());
    http_client_test!(status_unauthorized_should_succeed, "status/unauthorized.json", .expect_status_unauthorized());
    http_client_test!(status_unauthorized_should_fail, "status/ok.json", "expected status to be '401' but was '200'", .expect_status_unauthorized());
    http_client_test!(status_forbidden_should_succeed, "status/forbidden.json", .expect_status_forbidden());
    http_client_test!(status_forbidden_should_fail, "status/ok.json", "expected status to be '403' but was '200'", .expect_status_forbidden());
    http_client_test!(status_not_found_should_succeed, "status/not-found.json", .expect_status_not_found());
    http_client_test!(status_not_found_should_fail, "status/ok.json", "expected status to be '404' but was '200'", .expect_status_not_found());
    http_client_test!(status_conflict_should_succeed, "status/conflict.json", .expect_status_conflict());
    http_client_test!(status_conflict_should_fail, "status/ok.json", "expected status to be '409' but was '200'", .expect_status_conflict());
    http_client_test!(status_gone_should_succeed, "status/gone.json", .expect_status_gone());
    http_client_test!(status_gone_should_fail, "status/ok.json", "expected status to be '410' but was '200'", .expect_status_gone());
    http_client_test!(status_internal_server_error_should_succeed, "status/server-error.json", .expect_status_internal_server_error());
    http_client_test!(status_internal_server_error_should_fail, "status/ok.json", "expected status to be '500' but was '200'", .expect_status_internal_server_error());

    http_client_test!(status_range_should_succeed, "status/created.json", .expect_status_in_range(200, 300));
    http_client_test!(status_range_should_fail_when_higher, "status/server-error.json", "expected status to be in [300;400[ but was '500'", .expect_status_in_range(300, 400));
    http_client_test!(status_range_should_fail_when_lower, "status/ok.json", "expected status to be in [300;400[ but was '200'", .expect_status_in_range(300, 400));
    http_client_test!(status_range_should_succeed_in_inclusive_lower_range, "status/ok.json", .expect_status_in_range(200, 202));
    http_client_test!(status_range_should_fail_when_in_exclusive_upper_range, "status/accepted.json", "expected status to be in [200;202[ but was '202'", .expect_status_in_range(200, 202));

    http_client_test!(status_success_should_succeed, "status/ok.json", .expect_status_success());
    http_client_test!(status_success_should_fail, "status/bad-request.json", "expected status to be in [200;300[ but was '400'", .expect_status_success());
    http_client_test!(status_redirection_should_succeed, "status/moved-permanently.json", .expect_status_redirection());
    http_client_test!(status_redirection_should_fail, "status/server-error.json", "expected status to be in [300;400[ but was '500'", .expect_status_redirection());
    http_client_test!(status_client_error_should_succeed, "status/bad-request.json", .expect_status_client_error());
    http_client_test!(status_client_error_should_fail, "status/ok.json", "expected status to be in [400;500[ but was '200'", .expect_status_client_error());
    http_client_test!(status_server_error_should_succeed, "status/server-error.json", .expect_status_server_error());
    http_client_test!(status_server_error_should_fail, "status/ok.json", "expected status to be in [500;600[ but was '200'", .expect_status_server_error());
}

mod header {
    http_client_test!(header_eq_should_succeed, "header/one.json", .expect_header("x-a", "a"));
    http_client_test!(header_eq_should_match_key_ignoring_case, "header/one.json", .expect_header("X-A", "a"));
    http_client_test!(header_eq_should_fail_because_value_case_sensitive, "header/one.json", "expected header 'x-a' to be equal to 'A' but was 'a'", .expect_header("x-a", "A"));
    http_client_test!(header_eq_should_fail_when_wrong_key, "header/one.json", "expected one header named 'x-b' but none found", .expect_header("x-b", "a"));
    http_client_test!(header_eq_should_fail_when_wrong_value, "header/one.json", "expected header 'x-a' to be equal to 'b' but was 'a'", .expect_header("x-a", "b"));
    http_client_test!(header_eq_many_should_succeed, "header/many.json", .expect_header("x-a", "a").expect_header("x-b", "b"));
    http_client_test!(header_eq_should_fail_when_multivalued, "header/multi.json", "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.", .expect_header("x-m", "a"));

    http_client_test!(header_multi_should_succeed, "header/multi.json", .expect_headers("x-m", ["a", "b"]));
    http_client_test!(header_multi_should_fail_when_key_missing, "header/multi.json", "expected one header named 'x-b' but none found", .expect_headers("x-b", ["a", "b"]));
    http_client_test!(header_multi_should_fail_when_one_value_missing, "header/multi.json", "expected header 'x-m' to contain values '[\"a\"]' but contained '[\"a\", \"b\"]'", .expect_headers("x-m", ["a"]));
    http_client_test!(header_multi_should_fail_when_one_value_not_eq, "header/multi.json", "expected header 'x-m' to contain values '[\"a\", \"c\"]' but contained '[\"a\", \"b\"]'", .expect_headers("x-m", ["a", "c"]));
    http_client_test!(header_multi_should_fail_when_one_expected_value_missing, "header/multi.json", "expected header 'x-m' to contain values '[\"a\", \"b\", \"c\"]' but contained '[\"a\", \"b\"]'", .expect_headers("x-m", ["a", "b", "c"]));
    http_client_test!(header_multi_should_fail_when_no_value_expected, "header/multi.json", "no value expected for header 'x-m'. Use 'expect_header_present' instead", .expect_headers("x-m", []));

    http_client_test!(header_present_should_succeed, "header/one.json", .expect_header_present("x-a"));
    http_client_test!(header_present_should_succeed_ignoring_case, "header/one.json", .expect_header_present("X-A"));
    http_client_test!(header_present_should_fail_when_absent, "header/one.json", "expected one header named 'x-b' but none found", .expect_header_present("x-b"));

    http_client_test!(header_absent_should_succeed, "header/one.json", .expect_header_absent("x-b"));
    http_client_test!(header_absent_should_fail_when_present, "header/one.json", "expected no header named 'x-a' but one found", .expect_header_absent("x-a"));
    http_client_test!(header_absent_should_fail_when_present_ignoring_case, "header/one.json", "expected no header named 'x-a' but one found", .expect_header_absent("X-A"));

    http_client_test!(header_content_type_json_should_succeed, "header/json.json", .expect_content_type_json());
    http_client_test!(header_content_type_json_should_fail, "header/xml.json", "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'", .expect_content_type_json());

    http_client_test!(header_content_type_text_should_succeed, "header/text.json", .expect_content_type_text());
    http_client_test!(header_content_type_text_should_fail, "header/xml.json", "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'", .expect_content_type_text());
}

mod body {
    use serde_json::{json, Value};

    use super::*;

    http_client_test!(body_json_should_succeed, "body/json/value.json", .expect_body_json(|b: Value| assert_eq!(b, json!({"a": "b"}))));
    http_client_test!(body_json_struct_should_succeed, "body/json/value.json", .expect_body_json(|b: TestBody| assert_eq!(b, TestBody { a: String::from("b") })));
    http_client_test!(body_json_should_fail_when_closure_fails, "body/json/value.json", "", .expect_body_json(|b: Value| assert_eq!(b, json!({"a": "c"}))));

    http_client_test!(body_json_eq_should_succeed, "body/json/value.json", .expect_body_json_eq(json!({"a": "b"})));
    http_client_test!(body_json_struct_eq_should_succeed, "body/json/value.json", .expect_body_json_eq(TestBody { a: String::from("b") }));
    http_client_test!(body_json_eq_should_fail_when_not_eq, "body/json/value.json", "expected json body '{\"a\":\"b\"}' to be equal to '{\"a\":\"c\"}' but was not", .expect_body_json_eq(json!({"a": "c"})));
    http_client_test!(body_json_eq_should_fail_when_absent, "body/json/absent.json", "expected a json body but no response body was present", .expect_body_json_eq(json!({"a": "b"})));

    http_client_test!(body_text_should_succeed, "body/text/value.json", .expect_body_text(|b| assert_eq!(b, String::from("abcd"))));
    http_client_test!(body_text_should_fail_when_closure_fails, "body/text/value.json", "", .expect_body_text(|b| assert_eq!(b, String::from("dcba"))));

    http_client_test!(body_text_eq_should_succeed, "body/text/value.json", .expect_body_text_eq("abcd"));
    http_client_test!(body_text_eq_should_fail_when_not_eq, "body/text/value.json", "expected text body 'abcd' to be equal to 'dcba' but was not", .expect_body_text_eq("dcba"));
    http_client_test!(body_text_eq_should_fail_when_absent, "body/text/absent.json", "expected a text body but no response body was present", .expect_body_text_eq("abcd"));

    http_client_test!(body_text_regex_should_succeed, "body/text/value.json", .expect_body_text_matches("[a-d]+"));
    http_client_test!(body_text_regex_should_fail_when_does_not_match, "body/text/value.json", "expected text body 'abcd' to match regex '[e-h]+' but did not", .expect_body_text_matches("[e-h]+"));
    http_client_test!(body_text_regex_should_fail_when_absent, "body/text/absent.json", "expected a text body but no response body was present", .expect_body_text_matches("[a-d]+"));

    http_client_test!(body_bytes_should_succeed, "body/bytes/value.json", .expect_body_bytes(|b| assert_eq!(b, b"abcd")));
    http_client_test!(body_bytes_should_fail_when_closure_fails, "body/bytes/value.json", "", .expect_body_bytes(|b| assert_eq!(b, b"dcba")));

    http_client_test!(body_bytes_eq_should_succeed, "body/bytes/value.json", .expect_body_bytes_eq(b"abcd"));
    http_client_test!(body_bytes_eq_should_fail_when_not_eq, "body/bytes/value.json", "expected body '[97, 98, 99, 100]' to be equal to '[100, 99, 98, 97]' but was not", .expect_body_bytes_eq(b"dcba"));
    http_client_test!(body_bytes_eq_should_fail_when_absent, "body/bytes/absent.json", "expected a response body but no response body was present", .expect_body_bytes_eq(b"abcd"));

    http_client_test!(body_present_should_succeed, "body/bytes/value.json", .expect_body_present());
    http_client_test!(body_present_should_fail_when_absent, "body/bytes/absent.json", "expected a response body but no response body was present", .expect_body_present());

    http_client_test!(body_absent_should_succeed, "body/bytes/absent.json", .expect_body_absent());
    http_client_test!(body_absent_should_fail_when_present, "body/bytes/value.json", "expected no response body but a response body was present", .expect_body_absent());
}