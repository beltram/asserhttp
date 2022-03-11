use actix_web::{HttpRequest, HttpResponse, test::TestRequest};

use asserhttp::*;

#[actix_web::test]
async fn simple_should_work() {
    async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
    ok(TestRequest::get().to_http_request()).await.expect_status_eq(200);
}

#[should_panic]
#[actix_web::test]
async fn simple_should_panic() {
    async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
    ok(TestRequest::get().to_http_request()).await.expect_status_eq(100);
}

#[actix_web::test]
async fn simple_should_chain() {
    async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
    ok(TestRequest::get().to_http_request()).await.expect_status_eq(200).expect_status_eq(200);
}

#[actix_web::test]
async fn result_should_work() {
    async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
    ok(TestRequest::get().to_http_request()).await.expect_status_eq(200);
}

#[should_panic]
#[actix_web::test]
async fn result_should_panic() {
    async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
    ok(TestRequest::get().to_http_request()).await.expect_status_eq(100);
}

#[actix_web::test]
async fn result_should_chain() {
    async fn ok(_: HttpRequest) -> HttpResponse { HttpResponse::Ok().finish() }
    ok(TestRequest::get().to_http_request()).await.expect_status_eq(200).expect_status_eq(200);
}