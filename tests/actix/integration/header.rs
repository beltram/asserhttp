use actix_web::{App, HttpResponse, test::{call_service, init_service, TestRequest}, web};

use asserhttp::*;

mod eq {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_header() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-a", "a")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_header("x-a", "a");
    }

    #[actix_rt::test]
    async fn should_expect_header_ignoring_case() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-a", "a")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_header("X-A", "a");
    }

    #[should_panic(expected = "expected header 'x-a' to be equal to 'A' but was 'a'")]
    #[actix_rt::test]
    async fn expect_header_value_should_be_case_sensitive() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-a", "a")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_header("x-a", "A");
    }

    #[actix_rt::test]
    async fn should_expect_many_header() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-a", "a")).append_header(("x-b", "b")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await
            .expect_header("x-a", "a")
            .expect_header("x-b", "b");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[actix_rt::test]
    async fn expect_header_should_panic_when_wrong_key() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-a", "a")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_header("x-b", "a");
    }

    #[should_panic(expected = "expected header 'x-a' to be equal to 'b' but was 'a'")]
    #[actix_rt::test]
    async fn expect_header_should_panic_when_wrong_value() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-a", "a")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_header("x-a", "b");
    }

    #[should_panic(expected = "expected header 'x-m' to be single valued. Had '2' values '[\"a\", \"b\"]'. Use 'expect_headers' instead.")]
    #[actix_rt::test]
    async fn expect_header_should_panic_when_multi_valued() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-m", "a, b")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_header("x-m", "a");
    }
}

mod multi {
    use super::*;

    #[actix_rt::test]
    async fn expect_headers_should_expect_multi_headers() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-m", "a, b")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_headers("x-m", ["a", "b"]);
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[actix_rt::test]
    async fn expect_headers_should_fail_when_key_missing() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-m", "a, b")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_headers("x-b", ["a", "b"]);
    }

    #[should_panic(expected = "expected header 'x-m' to contain values '[\"a\"]' but contained '[\"a\", \"b\"]'")]
    #[actix_rt::test]
    async fn expect_headers_should_fail_when_one_value_missing() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-m", "a, b")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_headers("x-m", ["a"]);
    }

    #[should_panic(expected = "expected header 'x-m' to contain values '[\"a\", \"c\"]' but contained '[\"a\", \"b\"]'")]
    #[actix_rt::test]
    async fn expect_headers_should_fail_when_one_value_not_eq() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-m", "a, b")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_headers("x-m", ["a", "c"]);
    }

    #[should_panic(expected = "expected header 'x-m' to contain values '[\"a\", \"b\", \"c\"]' but contained '[\"a\", \"b\"]'")]
    #[actix_rt::test]
    async fn expect_headers_should_fail_when_one_expected_value_missing() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-m", "a, b")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_headers("x-m", ["a", "b", "c"]);
    }

    #[should_panic(expected = "no value expected for header 'x-m'. Use 'expect_header_present' instead")]
    #[actix_rt::test]
    async fn expect_headers_should_fail_when_no_value_expected() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-m", "a, b")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_headers("x-m", []);
    }
}

mod present {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_header_present() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-a", "a")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_header_present("x-a");
    }

    #[actix_rt::test]
    async fn should_expect_header_present_ignoring_case() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-a", "a")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_header_present("X-A");
    }

    #[should_panic(expected = "expected one header named 'x-b' but none found")]
    #[actix_rt::test]
    async fn expect_header_present_should_fail_when_absent() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-a", "a")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_header_present("x-b");
    }
}

mod absent {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_header_absent() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-a", "a")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_header_absent("x-b");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[actix_rt::test]
    async fn expect_header_absent_should_fail_when_absent() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-a", "a")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_header_absent("x-a");
    }

    #[should_panic(expected = "expected no header named 'x-a' but one found")]
    #[actix_rt::test]
    async fn expect_header_absent_should_ignore_case() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("x-a", "a")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_header_absent("X-A");
    }
}

mod json {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_content_type_json() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("content-type", "application/json")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_content_type_json();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'application/json' but was 'application/xml'")]
    #[actix_rt::test]
    async fn expect_header_should_panic_when_wrong_value() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("content-type", "application/xml")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_content_type_json();
    }
}

mod text {
    use super::*;

    #[actix_rt::test]
    async fn should_expect_content_type_text() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("content-type", "text/plain")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_content_type_text();
    }

    #[should_panic(expected = "expected header 'content-type' to be equal to 'text/plain' but was 'application/xml'")]
    #[actix_rt::test]
    async fn expect_header_should_panic_when_wrong_value() {
        let app = App::new()
            .route("/", web::get().to(|| async { HttpResponse::Ok().append_header(("content-type", "application/xml")).finish() }));
        call_service(&mut init_service(app).await, TestRequest::get().to_request()).await.expect_content_type_text();
    }
}