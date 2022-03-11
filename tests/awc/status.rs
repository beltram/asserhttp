use awc::Client;

use asserhttp::*;

mod eq {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/eq.json")]
    async fn should_expect_status_eq() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_eq(200);
    }

    #[actix_web::test]
    #[stubr::mock("status/eq.json")]
    async fn should_expect_status_eq_enum() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_eq(Status::Ok);
    }

    #[should_panic(expected = "expected status to be '100' but was '200'")]
    #[stubr::mock("status/eq.json")]
    #[actix_web::test]
    async fn expect_status_eq_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_eq(100);
    }

    #[actix_web::test]
    #[stubr::mock("status/eq.json")]
    async fn result_should_expect_status_eq() {
        Client::default().get(stubr.uri()).send().await.expect_status_eq(200);
    }

    #[should_panic(expected = "expected status to be '100' but was '200'")]
    #[stubr::mock("status/eq.json")]
    #[actix_web::test]
    async fn result_expect_status_eq_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_eq(100);
    }
}

mod ok {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/ok.json")]
    async fn should_expect_status_ok() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_ok();
    }


    #[should_panic(expected = "expected status to be '200' but was '201'")]
    #[stubr::mock("status/created.json")]
    #[actix_web::test]
    async fn expect_status_ok_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_ok();
    }

    #[actix_web::test]
    #[stubr::mock("status/ok.json")]
    async fn result_should_expect_status_ok() {
        Client::default().get(stubr.uri()).send().await.expect_status_ok();
    }

    #[should_panic(expected = "expected status to be '200' but was '201'")]
    #[stubr::mock("status/created.json")]
    #[actix_web::test]
    async fn result_expect_status_ok_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_ok();
    }
}

mod created {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/created.json")]
    async fn should_expect_status_created() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_created();
    }

    #[should_panic(expected = "expected status to be '201' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_created_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_created();
    }

    #[actix_web::test]
    #[stubr::mock("status/created.json")]
    async fn result_should_expect_status_created() {
        Client::default().get(stubr.uri()).send().await.expect_status_created();
    }

    #[should_panic(expected = "expected status to be '201' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_created_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_created();
    }
}

mod accepted {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/accepted.json")]
    async fn should_expect_status_accepted() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_accepted();
    }

    #[should_panic(expected = "expected status to be '202' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_accepted_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_accepted();
    }

    #[actix_web::test]
    #[stubr::mock("status/accepted.json")]
    async fn result_should_expect_status_accepted() {
        Client::default().get(stubr.uri()).send().await.expect_status_accepted();
    }

    #[should_panic(expected = "expected status to be '202' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_accepted_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_accepted();
    }
}

mod no_content {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/no-content.json")]
    async fn should_expect_status_no_content() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_no_content();
    }

    #[should_panic(expected = "expected status to be '204' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_no_content_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_no_content();
    }

    #[actix_web::test]
    #[stubr::mock("status/no-content.json")]
    async fn result_should_expect_status_no_content() {
        Client::default().get(stubr.uri()).send().await.expect_status_no_content();
    }

    #[should_panic(expected = "expected status to be '204' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_no_content_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_no_content();
    }
}

mod partial_content {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/partial-content.json")]
    async fn should_expect_status_partial_content() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_partial_content();
    }

    #[should_panic(expected = "expected status to be '206' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_partial_content_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_partial_content();
    }

    #[actix_web::test]
    #[stubr::mock("status/partial-content.json")]
    async fn result_should_expect_status_partial_content() {
        Client::default().get(stubr.uri()).send().await.expect_status_partial_content();
    }

    #[should_panic(expected = "expected status to be '206' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_partial_content_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_partial_content();
    }
}

mod bad_request {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/bad-request.json")]
    async fn should_expect_status_bad_request() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_bad_request();
    }

    #[should_panic(expected = "expected status to be '400' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_bad_request_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_bad_request();
    }

    #[actix_web::test]
    #[stubr::mock("status/bad-request.json")]
    async fn result_should_expect_status_bad_request() {
        Client::default().get(stubr.uri()).send().await.expect_status_bad_request();
    }

    #[should_panic(expected = "expected status to be '400' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_bad_request_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_bad_request();
    }
}

mod unauthorized {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/unauthorized.json")]
    async fn should_expect_status_unauthorized() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_unauthorized();
    }

    #[should_panic(expected = "expected status to be '401' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_unauthorized_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_unauthorized();
    }

    #[actix_web::test]
    #[stubr::mock("status/unauthorized.json")]
    async fn result_should_expect_status_unauthorized() {
        Client::default().get(stubr.uri()).send().await.expect_status_unauthorized();
    }

    #[should_panic(expected = "expected status to be '401' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_unauthorized_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_unauthorized();
    }
}

mod forbidden {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/forbidden.json")]
    async fn should_expect_status_forbidden() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_forbidden();
    }

    #[should_panic(expected = "expected status to be '403' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_forbidden_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_forbidden();
    }

    #[actix_web::test]
    #[stubr::mock("status/forbidden.json")]
    async fn result_should_expect_status_forbidden() {
        Client::default().get(stubr.uri()).send().await.expect_status_forbidden();
    }

    #[should_panic(expected = "expected status to be '403' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_forbidden_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_forbidden();
    }
}

mod not_found {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/not_found.json")]
    async fn should_expect_status_not_found() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_not_found();
    }

    #[should_panic(expected = "expected status to be '404' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_not_found_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_not_found();
    }

    #[actix_web::test]
    #[stubr::mock("status/not_found.json")]
    async fn result_should_expect_status_not_found() {
        Client::default().get(stubr.uri()).send().await.expect_status_not_found();
    }

    #[should_panic(expected = "expected status to be '404' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_not_found_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_not_found();
    }
}

mod conflict {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/conflict.json")]
    async fn should_expect_status_conflict() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_conflict();
    }

    #[should_panic(expected = "expected status to be '409' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_conflict_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_conflict();
    }

    #[actix_web::test]
    #[stubr::mock("status/conflict.json")]
    async fn result_should_expect_status_conflict() {
        Client::default().get(stubr.uri()).send().await.expect_status_conflict();
    }

    #[should_panic(expected = "expected status to be '409' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_conflict_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_conflict();
    }
}

mod gone {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/gone.json")]
    async fn should_expect_status_gone() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_gone();
    }

    #[should_panic(expected = "expected status to be '410' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_gone_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_gone();
    }

    #[actix_web::test]
    #[stubr::mock("status/gone.json")]
    async fn result_should_expect_status_gone() {
        Client::default().get(stubr.uri()).send().await.expect_status_gone();
    }

    #[should_panic(expected = "expected status to be '410' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_gone_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_gone();
    }
}

mod internal_server_error {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/server-error.json")]
    async fn should_expect_status_internal_server_error() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_internal_server_error();
    }

    #[should_panic(expected = "expected status to be '500' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_internal_server_error_should_panic() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_internal_server_error();
    }

    #[actix_web::test]
    #[stubr::mock("status/server-error.json")]
    async fn result_should_expect_status_internal_server_error() {
        Client::default().get(stubr.uri()).send().await.expect_status_internal_server_error();
    }

    #[should_panic(expected = "expected status to be '500' but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_internal_server_error_should_panic() {
        Client::default().get(stubr.uri()).send().await.expect_status_internal_server_error();
    }
}

mod range {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/created.json")]
    async fn should_expect_status_in_inclusive_lower_range() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_in_range(201, 300);
    }

    #[actix_web::test]
    #[stubr::mock("status/created.json")]
    async fn should_expect_status_in_exclusive_upper_range() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_in_range(200, 202);
    }

    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    #[stubr::mock("status/created.json")]
    #[actix_web::test]
    async fn expect_status_in_range_should_panic_when_lower() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_in_range(202, 300);
    }

    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    #[stubr::mock("status/created.json")]
    #[actix_web::test]
    async fn expect_status_in_range_should_panic_when_upper() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_in_range(200, 201);
    }

    #[actix_web::test]
    #[stubr::mock("status/created.json")]
    async fn result_should_expect_status_in_inclusive_lower_range() {
        Client::default().get(stubr.uri()).send().await.expect_status_in_range(201, 300);
    }

    #[actix_web::test]
    #[stubr::mock("status/created.json")]
    async fn result_should_expect_status_in_exclusive_upper_range() {
        Client::default().get(stubr.uri()).send().await.expect_status_in_range(200, 202);
    }

    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    #[stubr::mock("status/created.json")]
    #[actix_web::test]
    async fn result_expect_status_in_range_should_panic_when_lower() {
        Client::default().get(stubr.uri()).send().await.expect_status_in_range(202, 300);
    }

    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    #[stubr::mock("status/created.json")]
    #[actix_web::test]
    async fn result_expect_status_in_range_should_panic_when_upper() {
        Client::default().get(stubr.uri()).send().await.expect_status_in_range(200, 201);
    }
}

mod success {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/ok.json")]
    async fn should_expect_status_success() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_success();
    }

    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    #[stubr::mock("status/bad-request.json")]
    #[actix_web::test]
    async fn expect_status_success_should_panic_when_not() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_success();
    }

    #[actix_web::test]
    #[stubr::mock("status/ok.json")]
    async fn result_should_expect_status_success() {
        Client::default().get(stubr.uri()).send().await.expect_status_success();
    }

    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    #[stubr::mock("status/bad-request.json")]
    #[actix_web::test]
    async fn result_expect_status_success_should_panic_when_not() {
        Client::default().get(stubr.uri()).send().await.expect_status_success();
    }
}

mod client_error {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/bad-request.json")]
    async fn should_expect_status_client_error() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_client_error();
    }

    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_client_error_should_panic_when_not() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_client_error();
    }

    #[actix_web::test]
    #[stubr::mock("status/bad-request.json")]
    async fn result_should_expect_status_client_error() {
        Client::default().get(stubr.uri()).send().await.expect_status_client_error();
    }

    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_client_error_should_panic_when_not() {
        Client::default().get(stubr.uri()).send().await.expect_status_client_error();
    }
}

mod server_error {
    use super::*;

    #[actix_web::test]
    #[stubr::mock("status/server-error.json")]
    async fn should_expect_status_server_error() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_server_error();
    }

    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn expect_status_server_error_should_panic_when_not() {
        Client::default().get(stubr.uri()).send().await.unwrap().expect_status_server_error();
    }

    #[actix_web::test]
    #[stubr::mock("status/server-error.json")]
    async fn result_should_expect_status_server_error() {
        Client::default().get(stubr.uri()).send().await.expect_status_server_error();
    }

    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    #[stubr::mock("status/ok.json")]
    #[actix_web::test]
    async fn result_expect_status_server_error_should_panic_when_not() {
        Client::default().get(stubr.uri()).send().await.expect_status_server_error();
    }
}