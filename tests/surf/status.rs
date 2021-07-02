use surf::get;

use asserhttp::*;

mod eq {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/eq.json")]
    async fn should_expect_status_eq() {
        get(stubr.uri()).await.unwrap().expect_status_eq(200);
    }

    #[should_panic(expected = "expected status to be '100' but was '200'")]
    #[stubr::mock("status/eq.json")]
    #[async_std::test]
    async fn expect_status_eq_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_eq(100);
    }

    #[async_std::test]
    #[stubr::mock("status/eq.json")]
    async fn result_should_expect_status_eq() {
        get(stubr.uri()).await.expect_status_eq(200);
    }

    #[should_panic(expected = "expected status to be '100' but was '200'")]
    #[stubr::mock("status/eq.json")]
    #[async_std::test]
    async fn result_expect_status_eq_should_panic() {
        get(stubr.uri()).await.expect_status_eq(100);
    }
}

mod ok {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/ok.json")]
    async fn should_expect_status_ok() {
        get(stubr.uri()).await.unwrap().expect_status_ok();
    }


    #[should_panic(expected = "expected status to be '200' but was '201'")]
    #[stubr::mock("status/created.json")]
    #[async_std::test]
    async fn expect_status_ok_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_ok();
    }

    #[async_std::test]
    #[stubr::mock("status/ok.json")]
    async fn result_should_expect_status_ok() {
        get(stubr.uri()).await.expect_status_ok();
    }

    #[should_panic(expected = "expected status to be '200' but was '201'")]
    #[stubr::mock("status/created.json")]
    #[async_std::test]
    async fn result_expect_status_ok_should_panic() {
        get(stubr.uri()).await.expect_status_ok();
    }
}

mod created {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/created.json")]
    async fn should_expect_status_created() {
        get(stubr.uri()).await.unwrap().expect_status_created();
    }

    #[should_panic(expected = "expected status to be '201' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_created_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_created();
    }

    #[async_std::test]
    #[stubr::mock("status/created.json")]
    async fn result_should_expect_status_created() {
        get(stubr.uri()).await.expect_status_created();
    }

    #[should_panic(expected = "expected status to be '201' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_created_should_panic() {
        get(stubr.uri()).await.expect_status_created();
    }
}

mod accepted {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/accepted.json")]
    async fn should_expect_status_accepted() {
        get(stubr.uri()).await.unwrap().expect_status_accepted();
    }

    #[should_panic(expected = "expected status to be '202' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_accepted_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_accepted();
    }

    #[async_std::test]
    #[stubr::mock("status/accepted.json")]
    async fn result_should_expect_status_accepted() {
        get(stubr.uri()).await.expect_status_accepted();
    }

    #[should_panic(expected = "expected status to be '202' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_accepted_should_panic() {
        get(stubr.uri()).await.expect_status_accepted();
    }
}

mod no_content {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/no-content.json")]
    async fn should_expect_status_no_content() {
        get(stubr.uri()).await.unwrap().expect_status_no_content();
    }

    #[should_panic(expected = "expected status to be '204' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_no_content_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_no_content();
    }

    #[async_std::test]
    #[stubr::mock("status/no-content.json")]
    async fn result_should_expect_status_no_content() {
        get(stubr.uri()).await.expect_status_no_content();
    }

    #[should_panic(expected = "expected status to be '204' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_no_content_should_panic() {
        get(stubr.uri()).await.expect_status_no_content();
    }
}

mod partial_content {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/partial-content.json")]
    async fn should_expect_status_partial_content() {
        get(stubr.uri()).await.unwrap().expect_status_partial_content();
    }

    #[should_panic(expected = "expected status to be '206' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_partial_content_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_partial_content();
    }

    #[async_std::test]
    #[stubr::mock("status/partial-content.json")]
    async fn result_should_expect_status_partial_content() {
        get(stubr.uri()).await.expect_status_partial_content();
    }

    #[should_panic(expected = "expected status to be '206' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_partial_content_should_panic() {
        get(stubr.uri()).await.expect_status_partial_content();
    }
}

mod bad_request {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/bad-request.json")]
    async fn should_expect_status_bad_request() {
        get(stubr.uri()).await.unwrap().expect_status_bad_request();
    }

    #[should_panic(expected = "expected status to be '400' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_bad_request_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_bad_request();
    }

    #[async_std::test]
    #[stubr::mock("status/bad-request.json")]
    async fn result_should_expect_status_bad_request() {
        get(stubr.uri()).await.expect_status_bad_request();
    }

    #[should_panic(expected = "expected status to be '400' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_bad_request_should_panic() {
        get(stubr.uri()).await.expect_status_bad_request();
    }
}

mod unauthorized {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/unauthorized.json")]
    async fn should_expect_status_unauthorized() {
        get(stubr.uri()).await.unwrap().expect_status_unauthorized();
    }

    #[should_panic(expected = "expected status to be '401' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_unauthorized_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_unauthorized();
    }

    #[async_std::test]
    #[stubr::mock("status/unauthorized.json")]
    async fn result_should_expect_status_unauthorized() {
        get(stubr.uri()).await.expect_status_unauthorized();
    }

    #[should_panic(expected = "expected status to be '401' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_unauthorized_should_panic() {
        get(stubr.uri()).await.expect_status_unauthorized();
    }
}

mod forbidden {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/forbidden.json")]
    async fn should_expect_status_forbidden() {
        get(stubr.uri()).await.unwrap().expect_status_forbidden();
    }

    #[should_panic(expected = "expected status to be '403' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_forbidden_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_forbidden();
    }

    #[async_std::test]
    #[stubr::mock("status/forbidden.json")]
    async fn result_should_expect_status_forbidden() {
        get(stubr.uri()).await.expect_status_forbidden();
    }

    #[should_panic(expected = "expected status to be '403' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_forbidden_should_panic() {
        get(stubr.uri()).await.expect_status_forbidden();
    }
}

mod not_found {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/not_found.json")]
    async fn should_expect_status_not_found() {
        get(stubr.uri()).await.unwrap().expect_status_not_found();
    }

    #[should_panic(expected = "expected status to be '404' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_not_found_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_not_found();
    }

    #[async_std::test]
    #[stubr::mock("status/not_found.json")]
    async fn result_should_expect_status_not_found() {
        get(stubr.uri()).await.expect_status_not_found();
    }

    #[should_panic(expected = "expected status to be '404' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_not_found_should_panic() {
        get(stubr.uri()).await.expect_status_not_found();
    }
}

mod conflict {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/conflict.json")]
    async fn should_expect_status_conflict() {
        get(stubr.uri()).await.unwrap().expect_status_conflict();
    }

    #[should_panic(expected = "expected status to be '409' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_conflict_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_conflict();
    }

    #[async_std::test]
    #[stubr::mock("status/conflict.json")]
    async fn result_should_expect_status_conflict() {
        get(stubr.uri()).await.expect_status_conflict();
    }

    #[should_panic(expected = "expected status to be '409' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_conflict_should_panic() {
        get(stubr.uri()).await.expect_status_conflict();
    }
}

mod gone {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/gone.json")]
    async fn should_expect_status_gone() {
        get(stubr.uri()).await.unwrap().expect_status_gone();
    }

    #[should_panic(expected = "expected status to be '410' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_gone_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_gone();
    }

    #[async_std::test]
    #[stubr::mock("status/gone.json")]
    async fn result_should_expect_status_gone() {
        get(stubr.uri()).await.expect_status_gone();
    }

    #[should_panic(expected = "expected status to be '410' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_gone_should_panic() {
        get(stubr.uri()).await.expect_status_gone();
    }
}

mod internal_server_error {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/server-error.json")]
    async fn should_expect_status_internal_server_error() {
        get(stubr.uri()).await.unwrap().expect_status_internal_server_error();
    }

    #[should_panic(expected = "expected status to be '500' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_internal_server_error_should_panic() {
        get(stubr.uri()).await.unwrap().expect_status_internal_server_error();
    }

    #[async_std::test]
    #[stubr::mock("status/server-error.json")]
    async fn result_should_expect_status_internal_server_error() {
        get(stubr.uri()).await.expect_status_internal_server_error();
    }

    #[should_panic(expected = "expected status to be '500' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_internal_server_error_should_panic() {
        get(stubr.uri()).await.expect_status_internal_server_error();
    }
}

mod range {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/created.json")]
    async fn should_expect_status_in_inclusive_lower_range() {
        get(stubr.uri()).await.unwrap().expect_status_in_range(201, 300);
    }

    #[async_std::test]
    #[stubr::mock("status/created.json")]
    async fn should_expect_status_in_exclusive_upper_range() {
        get(stubr.uri()).await.unwrap().expect_status_in_range(200, 202);
    }

    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    #[stubr::mock("status/created.json")]
    #[async_std::test]
    async fn expect_status_in_range_should_panic_when_lower() {
        get(stubr.uri()).await.unwrap().expect_status_in_range(202, 300);
    }

    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    #[stubr::mock("status/created.json")]
    #[async_std::test]
    async fn expect_status_in_range_should_panic_when_upper() {
        get(stubr.uri()).await.unwrap().expect_status_in_range(200, 201);
    }

    #[async_std::test]
    #[stubr::mock("status/created.json")]
    async fn result_should_expect_status_in_inclusive_lower_range() {
        get(stubr.uri()).await.expect_status_in_range(201, 300);
    }

    #[async_std::test]
    #[stubr::mock("status/created.json")]
    async fn result_should_expect_status_in_exclusive_upper_range() {
        get(stubr.uri()).await.expect_status_in_range(200, 202);
    }

    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    #[stubr::mock("status/created.json")]
    #[async_std::test]
    async fn result_expect_status_in_range_should_panic_when_lower() {
        get(stubr.uri()).await.expect_status_in_range(202, 300);
    }

    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    #[stubr::mock("status/created.json")]
    #[async_std::test]
    async fn result_expect_status_in_range_should_panic_when_upper() {
        get(stubr.uri()).await.expect_status_in_range(200, 201);
    }
}

mod success {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/ok.json")]
    async fn should_expect_status_success() {
        get(stubr.uri()).await.unwrap().expect_status_success();
    }

    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    #[stubr::mock("status/bad-request.json")]
    #[async_std::test]
    async fn expect_status_success_should_panic_when_not() {
        get(stubr.uri()).await.unwrap().expect_status_success();
    }

    #[async_std::test]
    #[stubr::mock("status/ok.json")]
    async fn result_should_expect_status_success() {
        get(stubr.uri()).await.expect_status_success();
    }

    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    #[stubr::mock("status/bad-request.json")]
    #[async_std::test]
    async fn result_expect_status_success_should_panic_when_not() {
        get(stubr.uri()).await.expect_status_success();
    }
}

mod redirection {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/moved-permanently.json")]
    async fn should_expect_status_redirection() {
        get(stubr.uri()).await.unwrap().expect_status_redirection();
    }

    #[should_panic(expected = "expected status to be in [300;400[ but was '400'")]
    #[stubr::mock("status/bad-request.json")]
    #[async_std::test]
    async fn expect_status_redirection_should_panic_when_not() {
        get(stubr.uri()).await.unwrap().expect_status_redirection();
    }

    #[async_std::test]
    #[stubr::mock("status/moved-permanently.json")]
    async fn result_should_expect_status_redirection() {
        get(stubr.uri()).await.expect_status_redirection();
    }

    #[should_panic(expected = "expected status to be in [300;400[ but was '400'")]
    #[stubr::mock("status/bad-request.json")]
    #[async_std::test]
    async fn result_expect_status_redirection_should_panic_when_not() {
        get(stubr.uri()).await.expect_status_redirection();
    }
}

mod client_error {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/bad-request.json")]
    async fn should_expect_status_client_error() {
        get(stubr.uri()).await.unwrap().expect_status_client_error();
    }

    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_client_error_should_panic_when_not() {
        get(stubr.uri()).await.unwrap().expect_status_client_error();
    }

    #[async_std::test]
    #[stubr::mock("status/bad-request.json")]
    async fn result_should_expect_status_client_error() {
        get(stubr.uri()).await.expect_status_client_error();
    }

    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_client_error_should_panic_when_not() {
        get(stubr.uri()).await.expect_status_client_error();
    }
}

mod server_error {
    use super::*;

    #[async_std::test]
    #[stubr::mock("status/server-error.json")]
    async fn should_expect_status_server_error() {
        get(stubr.uri()).await.unwrap().expect_status_server_error();
    }

    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn expect_status_server_error_should_panic_when_not() {
        get(stubr.uri()).await.unwrap().expect_status_server_error();
    }

    #[async_std::test]
    #[stubr::mock("status/server-error.json")]
    async fn result_should_expect_status_server_error() {
        get(stubr.uri()).await.expect_status_server_error();
    }

    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[async_std::test]
    async fn result_expect_status_server_error_should_panic_when_not() {
        get(stubr.uri()).await.expect_status_server_error();
    }
}