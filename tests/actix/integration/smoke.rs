use actix_web::{App, HttpResponse, test::{call_service, init_service, TestRequest}, web};

use asserhttp::*;

#[actix_web::test]
async fn simple_should_work() {
    let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().await }));
    call_service(&init_service(app).await, TestRequest::get().to_request()).await.expect_status_eq(200);
}

#[should_panic]
#[actix_web::test]
async fn simple_should_panic() {
    let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().await }));
    call_service(&init_service(app).await, TestRequest::get().to_request()).await.expect_status_eq(100);
}

#[actix_web::test]
async fn simple_should_chain() {
    let app = App::new().route("/", web::get().to(|| async { HttpResponse::Ok().await }));
    call_service(&init_service(app).await, TestRequest::get().to_request()).await
        .expect_status_eq(200).expect_status_eq(200);
}