use isahc::get;
use stubr::Stubr;

use asserhttp::*;

mod eq {
    use super::*;

    #[test]
    fn should_expect_status_eq() {
        let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
        get(&srv.uri()).unwrap().expect_status_eq(200);
    }

    #[test]
    #[should_panic(expected = "expected status to be '100' but was '200'")]
    fn expect_status_eq_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
        get(&srv.uri()).unwrap().expect_status_eq(100);
    }

    #[test]
    fn result_should_expect_status_eq() {
        let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
        get(&srv.uri()).expect_status_eq(200);
    }

    #[test]
    #[should_panic(expected = "expected status to be '100' but was '200'")]
    fn result_expect_status_eq_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/eq.json");
        get(&srv.uri()).expect_status_eq(100);
    }
}

mod ok {
    use super::*;

    #[test]
    fn should_expect_status_ok() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().expect_status_ok();
    }

    #[test]
    #[should_panic(expected = "expected status to be '200' but was '201'")]
    fn expect_status_ok_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/created.json");
        get(&srv.uri()).unwrap().expect_status_ok();
    }

    #[test]
    fn result_should_expect_status_ok() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).expect_status_ok();
    }

    #[test]
    #[should_panic(expected = "expected status to be '200' but was '201'")]
    fn result_expect_status_ok_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/created.json");
        get(&srv.uri()).expect_status_ok();
    }
}

mod created {
    use super::*;

    #[test]
    fn should_assert_created() {
        let srv = Stubr::start_blocking("tests/stubs/status/created.json");
        get(&srv.uri()).unwrap().expect_status_created();
    }

    #[test]
    #[should_panic(expected = "expected status to be '201' but was '200'")]
    fn assert_created_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().expect_status_created();
    }

    #[test]
    fn result_should_assert_created() {
        let srv = Stubr::start_blocking("tests/stubs/status/created.json");
        get(&srv.uri()).expect_status_created();
    }

    #[test]
    #[should_panic(expected = "expected status to be '201' but was '200'")]
    fn result_assert_created_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).expect_status_created();
    }
}

mod accepted {
    use super::*;

    #[test]
    fn should_assert_accepted() {
        let srv = Stubr::start_blocking("tests/stubs/status/accepted.json");
        get(&srv.uri()).unwrap().expect_status_accepted();
    }

    #[test]
    #[should_panic(expected = "expected status to be '202' but was '200'")]
    fn assert_accepted_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().expect_status_accepted();
    }

    #[test]
    fn result_should_assert_accepted() {
        let srv = Stubr::start_blocking("tests/stubs/status/accepted.json");
        get(&srv.uri()).expect_status_accepted();
    }

    #[test]
    #[should_panic(expected = "expected status to be '202' but was '200'")]
    fn result_assert_accepted_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).expect_status_accepted();
    }
}

mod no_content {
    use super::*;

    #[test]
    fn should_assert_no_content() {
        let srv = Stubr::start_blocking("tests/stubs/status/no-content.json");
        get(&srv.uri()).unwrap().expect_status_no_content();
    }

    #[test]
    #[should_panic(expected = "expected status to be '204' but was '200'")]
    fn assert_no_content_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().expect_status_no_content();
    }

    #[test]
    fn result_should_assert_no_content() {
        let srv = Stubr::start_blocking("tests/stubs/status/no-content.json");
        get(&srv.uri()).expect_status_no_content();
    }

    #[test]
    #[should_panic(expected = "expected status to be '204' but was '200'")]
    fn result_assert_no_content_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).expect_status_no_content();
    }
}

mod partial_content {
    use super::*;

    #[test]
    fn should_assert_partial_content() {
        let srv = Stubr::start_blocking("tests/stubs/status/partial-content.json");
        get(&srv.uri()).unwrap().expect_status_partial_content();
    }

    #[test]
    #[should_panic(expected = "expected status to be '206' but was '200'")]
    fn assert_partial_content_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().expect_status_partial_content();
    }

    #[test]
    fn result_should_assert_partial_content() {
        let srv = Stubr::start_blocking("tests/stubs/status/partial-content.json");
        get(&srv.uri()).expect_status_partial_content();
    }

    #[test]
    #[should_panic(expected = "expected status to be '206' but was '200'")]
    fn result_assert_partial_content_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).expect_status_partial_content();
    }
}

mod bad_request {
    use super::*;

    #[test]
    fn should_assert_bad_request() {
        let srv = Stubr::start_blocking("tests/stubs/status/bad-request.json");
        get(&srv.uri()).unwrap().expect_status_bad_request();
    }

    #[test]
    #[should_panic(expected = "expected status to be '400' but was '200'")]
    fn assert_bad_request_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().expect_status_bad_request();
    }

    #[test]
    fn result_should_assert_bad_request() {
        let srv = Stubr::start_blocking("tests/stubs/status/bad-request.json");
        get(&srv.uri()).expect_status_bad_request();
    }

    #[test]
    #[should_panic(expected = "expected status to be '400' but was '200'")]
    fn result_assert_bad_request_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).expect_status_bad_request();
    }
}

mod unauthorized {
    use super::*;

    #[test]
    fn should_assert_unauthorized() {
        let srv = Stubr::start_blocking("tests/stubs/status/unauthorized.json");
        get(&srv.uri()).unwrap().expect_status_unauthorized();
    }

    #[test]
    #[should_panic(expected = "expected status to be '401' but was '200'")]
    fn assert_unauthorized_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().expect_status_unauthorized();
    }

    #[test]
    fn result_should_assert_unauthorized() {
        let srv = Stubr::start_blocking("tests/stubs/status/unauthorized.json");
        get(&srv.uri()).expect_status_unauthorized();
    }

    #[test]
    #[should_panic(expected = "expected status to be '401' but was '200'")]
    fn result_assert_unauthorized_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).expect_status_unauthorized();
    }
}

mod forbidden {
    use super::*;

    #[test]
    fn should_assert_forbidden() {
        let srv = Stubr::start_blocking("tests/stubs/status/forbidden.json");
        get(&srv.uri()).unwrap().expect_status_forbidden();
    }

    #[test]
    #[should_panic(expected = "expected status to be '403' but was '200'")]
    fn assert_forbidden_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().expect_status_forbidden();
    }

    #[test]
    fn result_should_assert_forbidden() {
        let srv = Stubr::start_blocking("tests/stubs/status/forbidden.json");
        get(&srv.uri()).expect_status_forbidden();
    }

    #[test]
    #[should_panic(expected = "expected status to be '403' but was '200'")]
    fn result_assert_forbidden_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).expect_status_forbidden();
    }
}

mod not_found {
    use super::*;

    #[test]
    fn should_assert_not_found() {
        let srv = Stubr::start_blocking("tests/stubs/status/not-found.json");
        get(&srv.uri()).unwrap().expect_status_not_found();
    }

    #[test]
    #[should_panic(expected = "expected status to be '404' but was '200'")]
    fn assert_not_found_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().expect_status_not_found();
    }

    #[test]
    fn result_should_assert_not_found() {
        let srv = Stubr::start_blocking("tests/stubs/status/not-found.json");
        get(&srv.uri()).expect_status_not_found();
    }

    #[test]
    #[should_panic(expected = "expected status to be '404' but was '200'")]
    fn result_assert_not_found_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).expect_status_not_found();
    }
}

mod conflict {
    use super::*;

    #[test]
    fn should_assert_conflict() {
        let srv = Stubr::start_blocking("tests/stubs/status/conflict.json");
        get(&srv.uri()).unwrap().expect_status_conflict();
    }

    #[test]
    #[should_panic(expected = "expected status to be '409' but was '200'")]
    fn assert_conflict_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().expect_status_conflict();
    }

    #[test]
    fn result_should_assert_conflict() {
        let srv = Stubr::start_blocking("tests/stubs/status/conflict.json");
        get(&srv.uri()).expect_status_conflict();
    }

    #[test]
    #[should_panic(expected = "expected status to be '409' but was '200'")]
    fn result_assert_conflict_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).expect_status_conflict();
    }
}

mod gone {
    use super::*;

    #[test]
    fn should_assert_gone() {
        let srv = Stubr::start_blocking("tests/stubs/status/gone.json");
        get(&srv.uri()).unwrap().expect_status_gone();
    }

    #[test]
    #[should_panic(expected = "expected status to be '410' but was '200'")]
    fn assert_gone_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().expect_status_gone();
    }

    #[test]
    fn result_should_assert_gone() {
        let srv = Stubr::start_blocking("tests/stubs/status/gone.json");
        get(&srv.uri()).expect_status_gone();
    }

    #[test]
    #[should_panic(expected = "expected status to be '410' but was '200'")]
    fn result_assert_gone_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).expect_status_gone();
    }
}

mod internal_server_error {
    use super::*;

    #[test]
    fn should_assert_server_error() {
        let srv = Stubr::start_blocking("tests/stubs/status/server-error.json");
        get(&srv.uri()).unwrap().expect_status_internal_server_error();
    }

    #[test]
    #[should_panic(expected = "expected status to be '500' but was '200'")]
    fn assert_server_error_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).unwrap().expect_status_internal_server_error();
    }

    #[test]
    fn result_should_assert_server_error() {
        let srv = Stubr::start_blocking("tests/stubs/status/server-error.json");
        get(&srv.uri()).expect_status_internal_server_error();
    }

    #[test]
    #[should_panic(expected = "expected status to be '500' but was '200'")]
    fn result_assert_server_error_should_panic() {
        let srv = Stubr::start_blocking("tests/stubs/status/ok.json");
        get(&srv.uri()).expect_status_internal_server_error();
    }
}

mod range {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_in_inclusive_lower_range() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).unwrap().expect_status_in_range(201, 300);
    }

    #[async_std::test]
    async fn should_expect_status_in_exclusive_upper_range() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).unwrap().expect_status_in_range(200, 202);
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    async fn expect_status_in_range_should_panic_when_lower() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).unwrap().expect_status_in_range(202, 300);
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    async fn expect_status_in_range_should_panic_when_upper() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).unwrap().expect_status_in_range(200, 201);
    }

    #[async_std::test]
    async fn result_should_expect_status_in_inclusive_lower_range() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).expect_status_in_range(201, 300);
    }

    #[async_std::test]
    async fn result_should_expect_status_in_exclusive_upper_range() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).expect_status_in_range(200, 202);
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    async fn result_expect_status_in_range_should_panic_when_lower() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).expect_status_in_range(202, 300);
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    async fn result_expect_status_in_range_should_panic_when_upper() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).expect_status_in_range(200, 201);
    }
}

mod success {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_success() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).unwrap().expect_status_success();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    async fn expect_status_success_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).unwrap().expect_status_success();
    }

    #[async_std::test]
    async fn result_should_expect_status_success() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).expect_status_success();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    async fn result_expect_status_success_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).expect_status_success();
    }
}

mod redirection {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_redirection() {
        let srv = Stubr::start("tests/stubs/status/moved-permanently.json").await;
        get(&srv.uri()).unwrap().expect_status_redirection();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [300;400[ but was '400'")]
    async fn expect_status_redirection_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).unwrap().expect_status_redirection();
    }

    #[async_std::test]
    async fn result_should_expect_status_redirection() {
        let srv = Stubr::start("tests/stubs/status/moved-permanently.json").await;
        get(&srv.uri()).expect_status_redirection();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [300;400[ but was '400'")]
    async fn result_expect_status_redirection_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).expect_status_redirection();
    }
}

mod client_error {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_client_error() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).unwrap().expect_status_client_error();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    async fn expect_status_client_error_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).unwrap().expect_status_client_error();
    }

    #[async_std::test]
    async fn result_should_expect_status_client_error() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).expect_status_client_error();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    async fn result_expect_status_client_error_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).expect_status_client_error();
    }
}

mod server_error {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_server_error() {
        let srv = Stubr::start("tests/stubs/status/server-error.json").await;
        get(&srv.uri()).unwrap().expect_status_server_error();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    async fn expect_status_server_error_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).unwrap().expect_status_server_error();
    }

    #[async_std::test]
    async fn result_should_expect_status_server_error() {
        let srv = Stubr::start("tests/stubs/status/server-error.json").await;
        get(&srv.uri()).expect_status_server_error();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    async fn result_expect_status_server_error_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).expect_status_server_error();
    }
}