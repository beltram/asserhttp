use stubr::Stubr;
use surf::get;

use asserhttp::*;

mod eq {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_eq() {
        let srv = Stubr::start("tests/stubs/status/eq.json").await;
        get(&srv.uri()).await.unwrap().expect_status_eq(200);
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '100' but was '200'")]
    async fn expect_status_eq_should_panic() {
        let srv = Stubr::start("tests/stubs/status/eq.json").await;
        get(&srv.uri()).await.unwrap().expect_status_eq(100);
    }

    #[async_std::test]
    async fn result_should_expect_status_eq() {
        let srv = Stubr::start("tests/stubs/status/eq.json").await;
        get(&srv.uri()).await.expect_status_eq(200);
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '100' but was '200'")]
    async fn result_expect_status_eq_should_panic() {
        let srv = Stubr::start("tests/stubs/status/eq.json").await;
        get(&srv.uri()).await.expect_status_eq(100);
    }
}

mod ok {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_ok() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_ok();
    }


    #[async_std::test]
    #[should_panic(expected = "expected status to be '200' but was '201'")]
    async fn expect_status_ok_should_panic() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).await.unwrap().expect_status_ok();
    }

    #[async_std::test]
    async fn result_should_expect_status_ok() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_ok();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '200' but was '201'")]
    async fn result_expect_status_ok_should_panic() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).await.expect_status_ok();
    }
}

mod created {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_created() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).await.unwrap().expect_status_created();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '201' but was '200'")]
    async fn expect_status_created_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_created();
    }

    #[async_std::test]
    async fn result_should_expect_status_created() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).await.expect_status_created();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '201' but was '200'")]
    async fn result_expect_status_created_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_created();
    }
}

mod accepted {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_accepted() {
        let srv = Stubr::start("tests/stubs/status/accepted.json").await;
        get(&srv.uri()).await.unwrap().expect_status_accepted();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '202' but was '200'")]
    async fn expect_status_accepted_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_accepted();
    }

    #[async_std::test]
    async fn result_should_expect_status_accepted() {
        let srv = Stubr::start("tests/stubs/status/accepted.json").await;
        get(&srv.uri()).await.expect_status_accepted();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '202' but was '200'")]
    async fn result_expect_status_accepted_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_accepted();
    }
}

mod no_content {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_no_content() {
        let srv = Stubr::start("tests/stubs/status/no-content.json").await;
        get(&srv.uri()).await.unwrap().expect_status_no_content();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '204' but was '200'")]
    async fn expect_status_no_content_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_no_content();
    }

    #[async_std::test]
    async fn result_should_expect_status_no_content() {
        let srv = Stubr::start("tests/stubs/status/no-content.json").await;
        get(&srv.uri()).await.expect_status_no_content();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '204' but was '200'")]
    async fn result_expect_status_no_content_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_no_content();
    }
}

mod partial_content {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_partial_content() {
        let srv = Stubr::start("tests/stubs/status/partial-content.json").await;
        get(&srv.uri()).await.unwrap().expect_status_partial_content();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '206' but was '200'")]
    async fn expect_status_partial_content_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_partial_content();
    }

    #[async_std::test]
    async fn result_should_expect_status_partial_content() {
        let srv = Stubr::start("tests/stubs/status/partial-content.json").await;
        get(&srv.uri()).await.expect_status_partial_content();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '206' but was '200'")]
    async fn result_expect_status_partial_content_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_partial_content();
    }
}

mod bad_request {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_bad_request() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).await.unwrap().expect_status_bad_request();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '400' but was '200'")]
    async fn expect_status_bad_request_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_bad_request();
    }

    #[async_std::test]
    async fn result_should_expect_status_bad_request() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).await.expect_status_bad_request();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '400' but was '200'")]
    async fn result_expect_status_bad_request_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_bad_request();
    }
}

mod unauthorized {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_unauthorized() {
        let srv = Stubr::start("tests/stubs/status/unauthorized.json").await;
        get(&srv.uri()).await.unwrap().expect_status_unauthorized();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '401' but was '200'")]
    async fn expect_status_unauthorized_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_unauthorized();
    }

    #[async_std::test]
    async fn result_should_expect_status_unauthorized() {
        let srv = Stubr::start("tests/stubs/status/unauthorized.json").await;
        get(&srv.uri()).await.expect_status_unauthorized();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '401' but was '200'")]
    async fn result_expect_status_unauthorized_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_unauthorized();
    }
}

mod forbidden {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_forbidden() {
        let srv = Stubr::start("tests/stubs/status/forbidden.json").await;
        get(&srv.uri()).await.unwrap().expect_status_forbidden();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '403' but was '200'")]
    async fn expect_status_forbidden_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_forbidden();
    }

    #[async_std::test]
    async fn result_should_expect_status_forbidden() {
        let srv = Stubr::start("tests/stubs/status/forbidden.json").await;
        get(&srv.uri()).await.expect_status_forbidden();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '403' but was '200'")]
    async fn result_expect_status_forbidden_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_forbidden();
    }
}

mod not_found {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_not_found() {
        let srv = Stubr::start("tests/stubs/status/not_found.json").await;
        get(&srv.uri()).await.unwrap().expect_status_not_found();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '404' but was '200'")]
    async fn expect_status_not_found_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_not_found();
    }

    #[async_std::test]
    async fn result_should_expect_status_not_found() {
        let srv = Stubr::start("tests/stubs/status/not_found.json").await;
        get(&srv.uri()).await.expect_status_not_found();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '404' but was '200'")]
    async fn result_expect_status_not_found_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_not_found();
    }
}

mod conflict {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_conflict() {
        let srv = Stubr::start("tests/stubs/status/conflict.json").await;
        get(&srv.uri()).await.unwrap().expect_status_conflict();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '409' but was '200'")]
    async fn expect_status_conflict_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_conflict();
    }

    #[async_std::test]
    async fn result_should_expect_status_conflict() {
        let srv = Stubr::start("tests/stubs/status/conflict.json").await;
        get(&srv.uri()).await.expect_status_conflict();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '409' but was '200'")]
    async fn result_expect_status_conflict_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_conflict();
    }
}

mod gone {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_gone() {
        let srv = Stubr::start("tests/stubs/status/gone.json").await;
        get(&srv.uri()).await.unwrap().expect_status_gone();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '410' but was '200'")]
    async fn expect_status_gone_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_gone();
    }

    #[async_std::test]
    async fn result_should_expect_status_gone() {
        let srv = Stubr::start("tests/stubs/status/gone.json").await;
        get(&srv.uri()).await.expect_status_gone();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '410' but was '200'")]
    async fn result_expect_status_gone_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_gone();
    }
}

mod internal_server_error {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_internal_server_error() {
        let srv = Stubr::start("tests/stubs/status/server-error.json").await;
        get(&srv.uri()).await.unwrap().expect_status_internal_server_error();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '500' but was '200'")]
    async fn expect_status_internal_server_error_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_internal_server_error();
    }

    #[async_std::test]
    async fn result_should_expect_status_internal_server_error() {
        let srv = Stubr::start("tests/stubs/status/server-error.json").await;
        get(&srv.uri()).await.expect_status_internal_server_error();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be '500' but was '200'")]
    async fn result_expect_status_internal_server_error_should_panic() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_internal_server_error();
    }
}

mod range {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_in_inclusive_lower_range() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).await.unwrap().expect_status_in_range(201, 300);
    }

    #[async_std::test]
    async fn should_expect_status_in_exclusive_upper_range() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).await.unwrap().expect_status_in_range(200, 202);
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    async fn expect_status_in_range_should_panic_when_lower() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).await.unwrap().expect_status_in_range(202, 300);
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    async fn expect_status_in_range_should_panic_when_upper() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).await.unwrap().expect_status_in_range(200, 201);
    }

    #[async_std::test]
    async fn result_should_expect_status_in_inclusive_lower_range() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).await.expect_status_in_range(201, 300);
    }

    #[async_std::test]
    async fn result_should_expect_status_in_exclusive_upper_range() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).await.expect_status_in_range(200, 202);
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    async fn result_expect_status_in_range_should_panic_when_lower() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).await.expect_status_in_range(202, 300);
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    async fn result_expect_status_in_range_should_panic_when_upper() {
        let srv = Stubr::start("tests/stubs/status/created.json").await;
        get(&srv.uri()).await.expect_status_in_range(200, 201);
    }
}

mod success {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_success() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_success();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    async fn expect_status_success_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).await.unwrap().expect_status_success();
    }

    #[async_std::test]
    async fn result_should_expect_status_success() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_success();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    async fn result_expect_status_success_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).await.expect_status_success();
    }
}

mod redirection {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_redirection() {
        let srv = Stubr::start("tests/stubs/status/moved-permanently.json").await;
        get(&srv.uri()).await.unwrap().expect_status_redirection();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [300;400[ but was '400'")]
    async fn expect_status_redirection_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).await.unwrap().expect_status_redirection();
    }

    #[async_std::test]
    async fn result_should_expect_status_redirection() {
        let srv = Stubr::start("tests/stubs/status/moved-permanently.json").await;
        get(&srv.uri()).await.expect_status_redirection();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [300;400[ but was '400'")]
    async fn result_expect_status_redirection_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).await.expect_status_redirection();
    }
}

mod client_error {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_client_error() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).await.unwrap().expect_status_client_error();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    async fn expect_status_client_error_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_client_error();
    }

    #[async_std::test]
    async fn result_should_expect_status_client_error() {
        let srv = Stubr::start("tests/stubs/status/bad-request.json").await;
        get(&srv.uri()).await.expect_status_client_error();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    async fn result_expect_status_client_error_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_client_error();
    }
}

mod server_error {
    use super::*;

    #[async_std::test]
    async fn should_expect_status_server_error() {
        let srv = Stubr::start("tests/stubs/status/server-error.json").await;
        get(&srv.uri()).await.unwrap().expect_status_server_error();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    async fn expect_status_server_error_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.unwrap().expect_status_server_error();
    }

    #[async_std::test]
    async fn result_should_expect_status_server_error() {
        let srv = Stubr::start("tests/stubs/status/server-error.json").await;
        get(&srv.uri()).await.expect_status_server_error();
    }

    #[async_std::test]
    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    async fn result_expect_status_server_error_should_panic_when_not() {
        let srv = Stubr::start("tests/stubs/status/ok.json").await;
        get(&srv.uri()).await.expect_status_server_error();
    }
}