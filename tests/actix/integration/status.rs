use actix_web::{App, HttpResponse, test::{call_service, init_service, TestRequest}, web};

use asserhttp::*;

mod eq {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_eq() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_eq(200);
    }

    #[should_panic(expected = "expected status to be '100' but was '200'")]
    #[actix_rt::test]
    async fn expect_status_eq_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_eq(100);
    }
}

mod ok {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_ok() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_ok();
    }

    #[should_panic(expected = "expected status to be '200' but was '201'")]
    #[actix_rt::test]
    async fn expect_status_ok_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Created() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_ok();
    }
}

mod created {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_created() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Created() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_created();
    }

    #[should_panic(expected = "expected status to be '201' but was '200'")]
    #[actix_rt::test]
    async fn expect_status_created_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_created();
    }
}

mod accepted {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_accepted() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Accepted() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_accepted();
    }

    #[should_panic(expected = "expected status to be '202' but was '200'")]
    #[actix_rt::test]
    async fn expect_status_accepted_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_accepted();
    }
}

mod no_content {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_no_content() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::NoContent() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_no_content();
    }

    #[should_panic(expected = "expected status to be '204' but was '200'")]
    #[actix_rt::test]
    async fn expect_status_no_content_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_no_content();
    }
}

mod partial_content {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_partial_content() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::PartialContent() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_partial_content();
    }

    #[should_panic(expected = "expected status to be '206' but was '200'")]
    #[actix_rt::test]
    async fn expect_status_partial_content_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_partial_content();
    }
}

mod bad_request {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_bad_request() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::BadRequest() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_bad_request();
    }

    #[should_panic(expected = "expected status to be '400' but was '200'")]
    #[actix_rt::test]
    async fn expect_status_bad_request_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_bad_request();
    }
}

mod unauthorized {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_unauthorized() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Unauthorized() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_unauthorized();
    }

    #[should_panic(expected = "expected status to be '401' but was '200'")]
    #[actix_rt::test]
    async fn expect_status_unauthorized_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_unauthorized();
    }
}

mod forbidden {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_forbidden() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Forbidden() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_forbidden();
    }

    #[should_panic(expected = "expected status to be '403' but was '200'")]
    #[actix_rt::test]
    async fn expect_status_forbidden_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_forbidden();
    }
}

mod not_found {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_not_found() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::NotFound() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_not_found();
    }

    #[should_panic(expected = "expected status to be '404' but was '200'")]
    #[actix_rt::test]
    async fn expect_status_not_found_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_not_found();
    }
}

mod conflict {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_conflict() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Conflict() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_conflict();
    }

    #[should_panic(expected = "expected status to be '409' but was '200'")]
    #[actix_rt::test]
    async fn expect_status_conflict_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_conflict();
    }
}

mod gone {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_gone() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Gone() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_gone();
    }

    #[should_panic(expected = "expected status to be '410' but was '200'")]
    #[actix_rt::test]
    async fn expect_status_gone_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_gone();
    }
}

mod internal_server_error {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_internal_server_error() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::InternalServerError() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_internal_server_error();
    }

    #[should_panic(expected = "expected status to be '500' but was '200'")]
    #[actix_rt::test]
    async fn expect_status_internal_server_error_should_panic() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_internal_server_error();
    }
}

mod range {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_in_inclusive_lower_range() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Created() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_in_range(201, 300);
    }

    #[actix_rt::test]
    async fn should_expect_status_in_exclusive_upper_range() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Created() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_in_range(200, 202);
    }

    #[should_panic(expected = "expected status to be in [202;300[ but was '201'")]
    #[actix_rt::test]
    async fn expect_status_in_range_should_panic_when_lower() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Created() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_in_range(202, 300);
    }

    #[should_panic(expected = "expected status to be in [200;201[ but was '201'")]
    #[actix_rt::test]
    async fn expect_status_in_range_should_panic_when_upper() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Created() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_in_range(200, 201);
    }
}

mod success {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_success() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_success();
    }

    #[should_panic(expected = "expected status to be in [200;300[ but was '400'")]
    #[actix_rt::test]
    async fn expect_status_success_should_panic_when_not() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::BadRequest() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_success();
    }
}

mod redirection {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_redirection() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::MovedPermanently() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_redirection();
    }

    #[should_panic(expected = "expected status to be in [300;400[ but was '400'")]
    #[actix_rt::test]
    async fn expect_status_redirection_should_panic_when_not() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::BadRequest() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_redirection();
    }
}

mod client_error {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_client_error() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::BadRequest() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_client_error();
    }

    #[should_panic(expected = "expected status to be in [400;500[ but was '200'")]
    #[actix_rt::test]
    async fn expect_status_client_error_should_panic_when_not() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_client_error();
    }
}

mod server_error {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_status_server_error() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::InternalServerError() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_server_error();
    }

    #[should_panic(expected = "expected status to be in [500;600[ but was '200'")]
    #[actix_rt::test]
    async fn expect_status_server_error_should_panic_when_not() {
        let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_status_server_error();
    }
}