use reqwest::blocking::get;

use asserhttp::*;

mod eq {
    use super::*;

    #[test]
    #[stubr::mock("status/eq.json")]
    fn should_expect_status_eq() {
        get(stubr.uri()).unwrap().expect_status_eq(200);
    }

    #[test]
    #[stubr::mock("status/eq.json")]
    fn should_expect_status_eq_enum() {
        get(stubr.uri()).unwrap().expect_status_eq(Status::Ok);
    }

    #[should_panic(expected = "expected status to be '100' but was '200'")]
    #[stubr::mock("status/eq.json")]
    #[test]
    fn expect_status_eq_should_panic() {
        get(stubr.uri()).unwrap().expect_status_eq(100);
    }

    #[test]
    #[stubr::mock("status/eq.json")]
    fn result_should_expect_status_eq() {
        get(stubr.uri()).expect_status_eq(200);
    }

    #[should_panic(expected = "expected status to be '100' but was '200'")]
    #[stubr::mock("status/eq.json")]
    #[test]
    fn result_expect_status_eq_should_panic() {
        get(stubr.uri()).expect_status_eq(100);
    }
}

mod ok {
    use super::*;

    #[test]
    #[stubr::mock("status/ok.json")]
    fn should_expect_status_ok() {
        get(stubr.uri()).unwrap().expect_status_ok();
    }

    #[should_panic(expected = "expected status to be '200' but was '201'")]
    #[stubr::mock("status/created.json")]
    #[test]
    fn expect_status_ok_should_panic() {
        get(stubr.uri()).unwrap().expect_status_ok();
    }

    #[test]
    #[stubr::mock("status/ok.json")]
    fn result_should_expect_status_ok() {
        get(stubr.uri()).expect_status_ok();
    }

    #[should_panic(expected = "expected status to be '200' but was '201'")]
    #[stubr::mock("status/created.json")]
    #[test]
    fn result_expect_status_ok_should_panic() {
        get(stubr.uri()).expect_status_ok();
    }
}

mod created {
    use super::*;

    #[test]
    #[stubr::mock("status/created.json")]
    fn should_assert_created() {
        get(stubr.uri()).unwrap().expect_status_created();
    }

    #[should_panic(expected = "expected status to be '201' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn assert_created_should_panic() {
        get(stubr.uri()).unwrap().expect_status_created();
    }

    #[test]
    #[stubr::mock("status/created.json")]
    fn result_should_assert_created() {
        get(stubr.uri()).expect_status_created();
    }

    #[should_panic(expected = "expected status to be '201' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_assert_created_should_panic() {
        get(stubr.uri()).expect_status_created();
    }
}

mod accepted {
    use super::*;

    #[test]
    #[stubr::mock("status/accepted.json")]
    fn should_assert_accepted() {
        get(stubr.uri()).unwrap().expect_status_accepted();
    }

    #[should_panic(expected = "expected status to be '202' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn assert_accepted_should_panic() {
        get(stubr.uri()).unwrap().expect_status_accepted();
    }

    #[test]
    #[stubr::mock("status/accepted.json")]
    fn result_should_assert_accepted() {
        get(stubr.uri()).expect_status_accepted();
    }

    #[should_panic(expected = "expected status to be '202' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_assert_accepted_should_panic() {
        get(stubr.uri()).expect_status_accepted();
    }
}

mod no_content {
    use super::*;

    #[test]
    #[stubr::mock("status/no-content.json")]
    fn should_assert_no_content() {
        get(stubr.uri()).unwrap().expect_status_no_content();
    }

    #[should_panic(expected = "expected status to be '204' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn assert_no_content_should_panic() {
        get(stubr.uri()).unwrap().expect_status_no_content();
    }

    #[test]
    #[stubr::mock("status/no-content.json")]
    fn result_should_assert_no_content() {
        get(stubr.uri()).expect_status_no_content();
    }

    #[should_panic(expected = "expected status to be '204' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_assert_no_content_should_panic() {
        get(stubr.uri()).expect_status_no_content();
    }
}

mod partial_content {
    use super::*;

    #[test]
    #[stubr::mock("status/partial-content.json")]
    fn should_assert_partial_content() {
        get(stubr.uri()).unwrap().expect_status_partial_content();
    }

    #[should_panic(expected = "expected status to be '206' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn assert_partial_content_should_panic() {
        get(stubr.uri()).unwrap().expect_status_partial_content();
    }

    #[test]
    #[stubr::mock("status/partial-content.json")]
    fn result_should_assert_partial_content() {
        get(stubr.uri()).expect_status_partial_content();
    }

    #[should_panic(expected = "expected status to be '206' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_assert_partial_content_should_panic() {
        get(stubr.uri()).expect_status_partial_content();
    }
}

mod bad_request {
    use super::*;

    #[test]
    #[stubr::mock("status/bad-request.json")]
    fn should_assert_bad_request() {
        get(stubr.uri()).unwrap().expect_status_bad_request();
    }

    #[should_panic(expected = "expected status to be '400' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn assert_bad_request_should_panic() {
        get(stubr.uri()).unwrap().expect_status_bad_request();
    }

    #[test]
    #[stubr::mock("status/bad-request.json")]
    fn result_should_assert_bad_request() {
        get(stubr.uri()).expect_status_bad_request();
    }

    #[should_panic(expected = "expected status to be '400' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_assert_bad_request_should_panic() {
        get(stubr.uri()).expect_status_bad_request();
    }
}

mod unauthorized {
    use super::*;

    #[test]
    #[stubr::mock("status/unauthorized.json")]
    fn should_assert_unauthorized() {
        get(stubr.uri()).unwrap().expect_status_unauthorized();
    }

    #[should_panic(expected = "expected status to be '401' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn assert_unauthorized_should_panic() {
        get(stubr.uri()).unwrap().expect_status_unauthorized();
    }

    #[test]
    #[stubr::mock("status/unauthorized.json")]
    fn result_should_assert_unauthorized() {
        get(stubr.uri()).expect_status_unauthorized();
    }

    #[should_panic(expected = "expected status to be '401' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_assert_unauthorized_should_panic() {
        get(stubr.uri()).expect_status_unauthorized();
    }
}

mod forbidden {
    use super::*;

    #[test]
    #[stubr::mock("status/forbidden.json")]
    fn should_assert_forbidden() {
        get(stubr.uri()).unwrap().expect_status_forbidden();
    }

    #[should_panic(expected = "expected status to be '403' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn assert_forbidden_should_panic() {
        get(stubr.uri()).unwrap().expect_status_forbidden();
    }

    #[test]
    #[stubr::mock("status/forbidden.json")]
    fn result_should_assert_forbidden() {
        get(stubr.uri()).expect_status_forbidden();
    }

    #[should_panic(expected = "expected status to be '403' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_assert_forbidden_should_panic() {
        get(stubr.uri()).expect_status_forbidden();
    }
}

mod not_found {
    use super::*;

    #[test]
    #[stubr::mock("status/not-found.json")]
    fn should_assert_not_found() {
        get(stubr.uri()).unwrap().expect_status_not_found();
    }

    #[should_panic(expected = "expected status to be '404' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn assert_not_found_should_panic() {
        get(stubr.uri()).unwrap().expect_status_not_found();
    }

    #[test]
    #[stubr::mock("status/not-found.json")]
    fn result_should_assert_not_found() {
        get(stubr.uri()).expect_status_not_found();
    }

    #[should_panic(expected = "expected status to be '404' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_assert_not_found_should_panic() {
        get(stubr.uri()).expect_status_not_found();
    }
}

mod conflict {
    use super::*;

    #[test]
    #[stubr::mock("status/conflict.json")]
    fn should_assert_conflict() {
        get(stubr.uri()).unwrap().expect_status_conflict();
    }

    #[should_panic(expected = "expected status to be '409' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn assert_conflict_should_panic() {
        get(stubr.uri()).unwrap().expect_status_conflict();
    }

    #[test]
    #[stubr::mock("status/conflict.json")]
    fn result_should_assert_conflict() {
        get(stubr.uri()).expect_status_conflict();
    }

    #[should_panic(expected = "expected status to be '409' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_assert_conflict_should_panic() {
        get(stubr.uri()).expect_status_conflict();
    }
}

mod gone {
    use super::*;

    #[test]
    #[stubr::mock("status/gone.json")]
    fn should_assert_gone() {
        get(stubr.uri()).unwrap().expect_status_gone();
    }

    #[should_panic(expected = "expected status to be '410' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn assert_gone_should_panic() {
        get(stubr.uri()).unwrap().expect_status_gone();
    }

    #[test]
    #[stubr::mock("status/gone.json")]
    fn result_should_assert_gone() {
        get(stubr.uri()).expect_status_gone();
    }

    #[should_panic(expected = "expected status to be '410' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_assert_gone_should_panic() {
        get(stubr.uri()).expect_status_gone();
    }
}

mod internal_server_error {
    use super::*;

    #[test]
    #[stubr::mock("status/server-error.json")]
    fn should_assert_server_error() {
        get(stubr.uri()).unwrap().expect_status_internal_server_error();
    }

    #[should_panic(expected = "expected status to be '500' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn assert_server_error_should_panic() {
        get(stubr.uri()).unwrap().expect_status_internal_server_error();
    }

    #[test]
    #[stubr::mock("status/server-error.json")]
    fn result_should_assert_server_error() {
        get(stubr.uri()).expect_status_internal_server_error();
    }

    #[should_panic(expected = "expected status to be '500' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_assert_server_error_should_panic() {
        get(stubr.uri()).expect_status_internal_server_error();
    }
}

mod range {
    use super::*;

    #[test]
    #[stubr::mock("status/created.json")]
    fn should_expect_status_in_inclusive_lower_range() {
        get(stubr.uri()).unwrap().expect_status_in_range(201, 300);
    }

    #[test]
    #[stubr::mock("status/created.json")]
    fn should_expect_status_in_exclusive_upper_range() {
        get(stubr.uri()).unwrap().expect_status_in_range(200, 202);
    }

    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    #[stubr::mock("status/created.json")]
    #[test]
    fn expect_status_in_range_should_panic_when_lower() {
        get(stubr.uri()).unwrap().expect_status_in_range(202, 300);
    }

    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    #[stubr::mock("status/created.json")]
    #[test]
    fn expect_status_in_range_should_panic_when_upper() {
        get(stubr.uri()).unwrap().expect_status_in_range(200, 201);
    }

    #[test]
    #[stubr::mock("status/created.json")]
    fn result_should_expect_status_in_inclusive_lower_range() {
        get(stubr.uri()).expect_status_in_range(201, 300);
    }

    #[test]
    #[stubr::mock("status/created.json")]
    fn result_should_expect_status_in_exclusive_upper_range() {
        get(stubr.uri()).expect_status_in_range(200, 202);
    }

    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    #[stubr::mock("status/created.json")]
    #[test]
    fn result_expect_status_in_range_should_panic_when_lower() {
        get(stubr.uri()).expect_status_in_range(202, 300);
    }

    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    #[stubr::mock("status/created.json")]
    #[test]
    fn result_expect_status_in_range_should_panic_when_upper() {
        get(stubr.uri()).expect_status_in_range(200, 201);
    }
}

mod success {
    use super::*;

    #[test]
    #[stubr::mock("status/ok.json")]
    fn should_expect_status_success() {
        get(stubr.uri()).unwrap().expect_status_success();
    }

    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    #[stubr::mock("status/bad-request.json")]
    #[test]
    fn expect_status_success_should_panic_when_not() {
        get(stubr.uri()).unwrap().expect_status_success();
    }

    #[test]
    #[stubr::mock("status/ok.json")]
    fn result_should_expect_status_success() {
        get(stubr.uri()).expect_status_success();
    }

    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    #[stubr::mock("status/bad-request.json")]
    #[test]
    fn result_expect_status_success_should_panic_when_not() {
        get(stubr.uri()).expect_status_success();
    }
}

mod redirection {
    use super::*;

    #[test]
    #[stubr::mock("status/moved-permanently.json")]
    fn should_expect_status_redirection() {
        get(stubr.uri()).unwrap().expect_status_redirection();
    }

    #[should_panic(expected = "expected status to be in [300;400[ but was '400'")]
    #[stubr::mock("status/bad-request.json")]
    #[test]
    fn expect_status_redirection_should_panic_when_not() {
        get(stubr.uri()).unwrap().expect_status_redirection();
    }

    #[test]
    #[stubr::mock("status/moved-permanently.json")]
    fn result_should_expect_status_redirection() {
        get(stubr.uri()).expect_status_redirection();
    }

    #[should_panic(expected = "expected status to be in [300;400[ but was '400'")]
    #[stubr::mock("status/bad-request.json")]
    #[test]
    fn result_expect_status_redirection_should_panic_when_not() {
        get(stubr.uri()).expect_status_redirection();
    }
}

mod client_error {
    use super::*;

    #[test]
    #[stubr::mock("status/bad-request.json")]
    fn should_expect_status_client_error() {
        get(stubr.uri()).unwrap().expect_status_client_error();
    }

    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn expect_status_client_error_should_panic_when_not() {
        get(stubr.uri()).unwrap().expect_status_client_error();
    }

    #[test]
    #[stubr::mock("status/bad-request.json")]
    fn result_should_expect_status_client_error() {
        get(stubr.uri()).expect_status_client_error();
    }

    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_expect_status_client_error_should_panic_when_not() {
        get(stubr.uri()).expect_status_client_error();
    }
}

mod server_error {
    use super::*;

    #[test]
    #[stubr::mock("status/server-error.json")]
    fn should_expect_status_server_error() {
        get(stubr.uri()).unwrap().expect_status_server_error();
    }

    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn expect_status_server_error_should_panic_when_not() {
        get(stubr.uri()).unwrap().expect_status_server_error();
    }

    #[test]
    #[stubr::mock("status/server-error.json")]
    fn result_should_expect_status_server_error() {
        get(stubr.uri()).expect_status_server_error();
    }

    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[test]
    fn result_expect_status_server_error_should_panic_when_not() {
        get(stubr.uri()).expect_status_server_error();
    }
}