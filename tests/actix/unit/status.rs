use actix_web::{HttpRequest, HttpResponse, test::TestRequest};

use asserhttp::*;

mod eq {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_eq() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_eq(200);
    }

    #[actix_web::test]
    async fn should_expect_status_eq_enum() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_eq(Status::Ok);
    }

    #[should_panic(expected = "expected status to be '100' but was '200'")]
    #[actix_web::test]
    async fn expect_status_eq_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_eq(100);
    }

    #[actix_web::test]
    async fn result_should_expect_status_eq() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_eq(200);
    }

    #[should_panic(expected = "expected status to be '100' but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_eq_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_eq(100);
    }
}

mod ok {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_ok() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_ok();
    }

    #[should_panic(expected = "expected status to be '200' but was '201'")]
    #[actix_web::test]
    async fn expect_status_ok_should_panic() {
        async fn created(_: HttpRequest) -> HttpResponse { HttpResponse::Created().finish() }
        created(TestRequest::get().to_http_request()).await.expect_status_ok();
    }

    #[actix_web::test]
    async fn result_should_expect_status_ok() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_ok();
    }

    #[should_panic(expected = "expected status to be '200' but was '201'")]
    #[actix_web::test]
    async fn result_expect_status_ok_should_panic() {
        async fn created(_: HttpRequest) -> HttpResponse { HttpResponse::Created().finish() }
        created(TestRequest::get().to_http_request()).await.expect_status_ok();
    }
}

mod created {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_created() {
        async fn created(_: HttpRequest) -> HttpResponse { HttpResponse::Created().finish() }
        created(TestRequest::get().to_http_request()).await.expect_status_created();
    }

    #[should_panic(expected = "expected status to be '201' but was '200'")]
    #[actix_web::test]
    async fn expect_status_created_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_created();
    }

    #[actix_web::test]
    async fn result_should_expect_status_created() {
        async fn created(_: HttpRequest) -> HttpResponse { HttpResponse::Created().finish() }
        created(TestRequest::get().to_http_request()).await.expect_status_created();
    }

    #[should_panic(expected = "expected status to be '201' but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_created_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_created();
    }
}

mod accepted {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_accepted() {
        async fn accepted(_: HttpRequest) -> HttpResponse { HttpResponse::Accepted().finish() }
        accepted(TestRequest::get().to_http_request()).await.expect_status_accepted();
    }

    #[should_panic(expected = "expected status to be '202' but was '200'")]
    #[actix_web::test]
    async fn expect_status_accepted_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_accepted();
    }

    #[actix_web::test]
    async fn result_should_expect_status_accepted() {
        async fn accepted(_: HttpRequest) -> HttpResponse { HttpResponse::Accepted().finish() }
        accepted(TestRequest::get().to_http_request()).await.expect_status_accepted();
    }

    #[should_panic(expected = "expected status to be '202' but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_accepted_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_accepted();
    }
}

mod no_content {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_no_content() {
        async fn no_content(_: HttpRequest) -> HttpResponse { HttpResponse::NoContent().finish() }
        no_content(TestRequest::get().to_http_request()).await.expect_status_no_content();
    }

    #[should_panic(expected = "expected status to be '204' but was '200'")]
    #[actix_web::test]
    async fn expect_status_no_content_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_no_content();
    }

    #[actix_web::test]
    async fn result_should_expect_status_no_content() {
        async fn no_content(_: HttpRequest) -> HttpResponse { HttpResponse::NoContent().finish() }
        no_content(TestRequest::get().to_http_request()).await.expect_status_no_content();
    }

    #[should_panic(expected = "expected status to be '204' but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_no_content_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_no_content();
    }
}

mod partial_content {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_partial_content() {
        async fn partial_content(_: HttpRequest) -> HttpResponse { HttpResponse::PartialContent().finish() }
        partial_content(TestRequest::get().to_http_request()).await.expect_status_partial_content();
    }

    #[should_panic(expected = "expected status to be '206' but was '200'")]
    #[actix_web::test]
    async fn expect_status_partial_content_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_partial_content();
    }

    #[actix_web::test]
    async fn result_should_expect_status_partial_content() {
        async fn partial_content(_: HttpRequest) -> HttpResponse { HttpResponse::PartialContent().finish() }
        partial_content(TestRequest::get().to_http_request()).await.expect_status_partial_content();
    }

    #[should_panic(expected = "expected status to be '206' but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_partial_content_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_partial_content();
    }
}

mod bad_request {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_bad_request() {
        async fn bad_request(_: HttpRequest) -> HttpResponse { HttpResponse::BadRequest().finish() }
        bad_request(TestRequest::get().to_http_request()).await.expect_status_bad_request();
    }

    #[should_panic(expected = "expected status to be '400' but was '200'")]
    #[actix_web::test]
    async fn expect_status_bad_request_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_bad_request();
    }

    #[actix_web::test]
    async fn result_should_expect_status_bad_request() {
        async fn bad_request(_: HttpRequest) -> HttpResponse { HttpResponse::BadRequest().finish() }
        bad_request(TestRequest::get().to_http_request()).await.expect_status_bad_request();
    }

    #[should_panic(expected = "expected status to be '400' but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_bad_request_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_bad_request();
    }
}

mod unauthorized {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_unauthorized() {
        async fn unauthorized(_: HttpRequest) -> HttpResponse { HttpResponse::Unauthorized().finish() }
        unauthorized(TestRequest::get().to_http_request()).await.expect_status_unauthorized();
    }

    #[should_panic(expected = "expected status to be '401' but was '200'")]
    #[actix_web::test]
    async fn expect_status_unauthorized_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_unauthorized();
    }

    #[actix_web::test]
    async fn result_should_expect_status_unauthorized() {
        async fn unauthorized(_: HttpRequest) -> HttpResponse { HttpResponse::Unauthorized().finish() }
        unauthorized(TestRequest::get().to_http_request()).await.expect_status_unauthorized();
    }

    #[should_panic(expected = "expected status to be '401' but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_unauthorized_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_unauthorized();
    }
}

mod forbidden {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_forbidden() {
        async fn forbidden(_: HttpRequest) -> HttpResponse { HttpResponse::Forbidden().finish() }
        forbidden(TestRequest::get().to_http_request()).await.expect_status_forbidden();
    }

    #[should_panic(expected = "expected status to be '403' but was '200'")]
    #[actix_web::test]
    async fn expect_status_forbidden_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_forbidden();
    }

    #[actix_web::test]
    async fn result_should_expect_status_forbidden() {
        async fn forbidden(_: HttpRequest) -> HttpResponse { HttpResponse::Forbidden().finish() }
        forbidden(TestRequest::get().to_http_request()).await.expect_status_forbidden();
    }

    #[should_panic(expected = "expected status to be '403' but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_forbidden_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_forbidden();
    }
}

mod not_found {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_not_found() {
        async fn not_found(_: HttpRequest) -> HttpResponse { HttpResponse::NotFound().finish() }
        not_found(TestRequest::get().to_http_request()).await.expect_status_not_found();
    }

    #[should_panic(expected = "expected status to be '404' but was '200'")]
    #[actix_web::test]
    async fn expect_status_not_found_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_not_found();
    }

    #[actix_web::test]
    async fn result_should_expect_status_not_found() {
        async fn not_found(_: HttpRequest) -> HttpResponse { HttpResponse::NotFound().finish() }
        not_found(TestRequest::get().to_http_request()).await.expect_status_not_found();
    }

    #[should_panic(expected = "expected status to be '404' but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_not_found_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_not_found();
    }
}

mod conflict {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_conflict() {
        async fn conflict(_: HttpRequest) -> HttpResponse { HttpResponse::Conflict().finish() }
        conflict(TestRequest::get().to_http_request()).await.expect_status_conflict();
    }

    #[should_panic(expected = "expected status to be '409' but was '200'")]
    #[actix_web::test]
    async fn expect_status_conflict_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_conflict();
    }

    #[actix_web::test]
    async fn result_should_expect_status_conflict() {
        async fn conflict(_: HttpRequest) -> HttpResponse { HttpResponse::Conflict().finish() }
        conflict(TestRequest::get().to_http_request()).await.expect_status_conflict();
    }

    #[should_panic(expected = "expected status to be '409' but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_conflict_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_conflict();
    }
}

mod gone {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_gone() {
        async fn gone(_: HttpRequest) -> HttpResponse { HttpResponse::Gone().finish() }
        gone(TestRequest::get().to_http_request()).await.expect_status_gone();
    }

    #[should_panic(expected = "expected status to be '410' but was '200'")]
    #[actix_web::test]
    async fn expect_status_gone_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_gone();
    }

    #[actix_web::test]
    async fn result_should_expect_status_gone() {
        async fn gone(_: HttpRequest) -> HttpResponse { HttpResponse::Gone().finish() }
        gone(TestRequest::get().to_http_request()).await.expect_status_gone();
    }

    #[should_panic(expected = "expected status to be '410' but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_gone_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_gone();
    }
}

mod internal_server_error {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_internal_server_error() {
        async fn internal_server_error(_: HttpRequest) -> HttpResponse { HttpResponse::InternalServerError().finish() }
        internal_server_error(TestRequest::get().to_http_request()).await.expect_status_internal_server_error();
    }

    #[should_panic(expected = "expected status to be '500' but was '200'")]
    #[actix_web::test]
    async fn expect_status_internal_server_error_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_internal_server_error();
    }

    #[actix_web::test]
    async fn result_should_expect_status_internal_server_error() {
        async fn internal_server_error(_: HttpRequest) -> HttpResponse { HttpResponse::InternalServerError().finish() }
        internal_server_error(TestRequest::get().to_http_request()).await.expect_status_internal_server_error();
    }

    #[should_panic(expected = "expected status to be '500' but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_internal_server_error_should_panic() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_internal_server_error();
    }
}

mod range {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_in_inclusive_lower_range() {
        async fn created(_: HttpRequest) -> HttpResponse { HttpResponse::Created().finish() }
        created(TestRequest::get().to_http_request()).await.expect_status_in_range(201, 300);
    }

    #[actix_web::test]
    async fn should_expect_status_in_exclusive_upper_range() {
        async fn created(_: HttpRequest) -> HttpResponse { HttpResponse::Created().finish() }
        created(TestRequest::get().to_http_request()).await.expect_status_in_range(200, 202);
    }

    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    #[actix_web::test]
    async fn expect_status_in_range_should_panic_when_lower() {
        async fn created(_: HttpRequest) -> HttpResponse { HttpResponse::Created().finish() }
        created(TestRequest::get().to_http_request()).await.expect_status_in_range(202, 300);
    }

    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    #[actix_web::test]
    async fn expect_status_in_range_should_panic_when_upper() {
        async fn created(_: HttpRequest) -> HttpResponse { HttpResponse::Created().finish() }
        created(TestRequest::get().to_http_request()).await.expect_status_in_range(200, 201);
    }

    #[actix_web::test]
    async fn result_should_expect_status_in_inclusive_lower_range() {
        async fn created(_: HttpRequest) -> HttpResponse { HttpResponse::Created().finish() }
        created(TestRequest::get().to_http_request()).await.expect_status_in_range(201, 300);
    }

    #[actix_web::test]
    async fn result_should_expect_status_in_exclusive_upper_range() {
        async fn created(_: HttpRequest) -> HttpResponse { HttpResponse::Created().finish() }
        created(TestRequest::get().to_http_request()).await.expect_status_in_range(200, 202);
    }

    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    #[actix_web::test]
    async fn result_expect_status_in_range_should_panic_when_lower() {
        async fn created(_: HttpRequest) -> HttpResponse { HttpResponse::Created().finish() }
        created(TestRequest::get().to_http_request()).await.expect_status_in_range(202, 300);
    }

    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    #[actix_web::test]
    async fn result_expect_status_in_range_should_panic_when_upper() {
        async fn created(_: HttpRequest) -> HttpResponse { HttpResponse::Created().finish() }
        created(TestRequest::get().to_http_request()).await.expect_status_in_range(200, 201);
    }
}

mod success {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_success() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_success();
    }

    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    #[actix_web::test]
    async fn expect_status_success_should_panic_when_not() {
        async fn bad_request(_: HttpRequest) -> HttpResponse { HttpResponse::BadRequest().finish() }
        bad_request(TestRequest::get().to_http_request()).await.expect_status_success();
    }

    #[actix_web::test]
    async fn result_should_expect_status_success() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_success();
    }

    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    #[actix_web::test]
    async fn result_expect_status_success_should_panic_when_not() {
        async fn bad_request(_: HttpRequest) -> HttpResponse { HttpResponse::BadRequest().finish() }
        bad_request(TestRequest::get().to_http_request()).await.expect_status_success();
    }
}

mod redirection {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_redirection() {
        async fn moved_permanently(_: HttpRequest) -> HttpResponse { HttpResponse::MovedPermanently().finish() }
        moved_permanently(TestRequest::get().to_http_request()).await.expect_status_redirection();
    }

    #[should_panic(expected = "expected status to be in [300;400[ but was '400'")]
    #[actix_web::test]
    async fn expect_status_redirection_should_panic_when_not() {
        async fn bad_request(_: HttpRequest) -> HttpResponse { HttpResponse::BadRequest().finish() }
        bad_request(TestRequest::get().to_http_request()).await.expect_status_redirection();
    }

    #[actix_web::test]
    async fn result_should_expect_status_redirection() {
        async fn moved_permanently(_: HttpRequest) -> HttpResponse { HttpResponse::MovedPermanently().finish() }
        moved_permanently(TestRequest::get().to_http_request()).await.expect_status_redirection();
    }

    #[should_panic(expected = "expected status to be in [300;400[ but was '400'")]
    #[actix_web::test]
    async fn result_expect_status_redirection_should_panic_when_not() {
        async fn bad_request(_: HttpRequest) -> HttpResponse { HttpResponse::BadRequest().finish() }
        bad_request(TestRequest::get().to_http_request()).await.expect_status_redirection();
    }
}

mod client_error {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_client_error() {
        async fn bad_request(_: HttpRequest) -> HttpResponse { HttpResponse::BadRequest().finish() }
        bad_request(TestRequest::get().to_http_request()).await.expect_status_client_error();
    }

    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    #[actix_web::test]
    async fn expect_status_client_error_should_panic_when_not() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_client_error();
    }

    #[actix_web::test]
    async fn result_should_expect_status_client_error() {
        async fn bad_request(_: HttpRequest) -> HttpResponse { HttpResponse::BadRequest().finish() }
        bad_request(TestRequest::get().to_http_request()).await.expect_status_client_error();
    }

    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_client_error_should_panic_when_not() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_client_error();
    }
}

mod server_error {
    use super::*;

    #[actix_web::test]
    async fn should_expect_status_server_error() {
        async fn internal_server_error(_: HttpRequest) -> HttpResponse { HttpResponse::InternalServerError().finish() }
        internal_server_error(TestRequest::get().to_http_request()).await.expect_status_server_error();
    }

    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    #[actix_web::test]
    async fn expect_status_server_error_should_panic_when_not() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_server_error();
    }

    #[actix_web::test]
    async fn result_should_expect_status_server_error() {
        async fn internal_server_error(_: HttpRequest) -> HttpResponse { HttpResponse::InternalServerError().finish() }
        internal_server_error(TestRequest::get().to_http_request()).await.expect_status_server_error();
    }

    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    #[actix_web::test]
    async fn result_expect_status_server_error_should_panic_when_not() {
        async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
        ok(TestRequest::get().to_http_request()).await.expect_status_server_error();
    }
}